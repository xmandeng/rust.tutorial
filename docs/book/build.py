"""Build the Rust textbook PDF from book.src.html.

Preprocessor directives (each on its own line in book.src.html):

  :::rs [label]        ... :::    Rust block written inline
  :::py [label]        ... :::    Python block written inline
  :::out [label]       ... :::    program or compiler output
  :::sh [label]        ... :::    shell commands
  :::toml [label]      ... :::    Cargo.toml excerpt
  :::json, :::yaml     ... :::    other config written inline
  :::include PATH                 a whole file, shown verbatim (config files)
  :::file PATH :: ANCHOR [:: ANCHOR ...]
      Excerpt from a real source file. For each ANCHOR, take the first line
      containing it, the comment and #[attribute] lines directly above it,
      and the rest of its { } block or its statement (brackets balanced).
      The separator is " :: " with spaces, so anchors may contain Rust paths
      such as "use tutorial::".

Run from the repo root:
  uv run --no-project --with weasyprint python docs/book/build.py
"""

import html
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent

RS_KW = set("""as async await break const continue crate dyn else enum false fn for if impl in
let loop match mod move mut pub ref return self Self static struct super trait true type unsafe
use where while""".split())
PY_KW = set("""and as async await class def elif else except False finally for from if import
in is lambda None not or pass raise return self True try while with yield match case""".split())

TOKEN = {
    "rs": re.compile(r'(?P<cm>//[^\n]*)|(?P<st>"(?:[^"\\\n]|\\.)*"|\'(?:[^\'\\\n]|\\.)\')'
                     r'|(?P<id>[A-Za-z_][A-Za-z0-9_]*)'),
    "py": re.compile(r'(?P<cm>#[^\n]*)|(?P<st>[fr]?"(?:[^"\\\n]|\\.)*"|[fr]?\'(?:[^\'\\\n]|\\.)*\')'
                     r'|(?P<id>[A-Za-z_][A-Za-z0-9_]*)'),
}
LONG_BLOCK_LINES = 22
LABELS = {"rs": "Rust", "py": "Python", "out": "Output", "sh": "Shell", "toml": "TOML",
          "json": "JSON", "yaml": "YAML"}


def highlight(code: str, lang: str) -> str:
    if lang not in TOKEN:
        return html.escape(code)
    kws = RS_KW if lang == "rs" else PY_KW
    out, pos = [], 0
    for m in TOKEN[lang].finditer(code):
        out.append(html.escape(code[pos:m.start()]))
        text = html.escape(m.group())
        if m.lastgroup == "cm":
            out.append(f'<span class="cm">{text}</span>')
        elif m.lastgroup == "st":
            out.append(f'<span class="st">{text}</span>')
        elif m.group() in kws:
            out.append(f'<span class="kw">{text}</span>')
        else:
            out.append(text)
        pos = m.end()
    out.append(html.escape(code[pos:]))
    return "".join(out)


def block(code: str, lang: str, label: str, may_split: bool = True) -> str:
    code = code.strip("\n")
    hd = f'<div class="codehd {lang}">{html.escape(label)}</div>' if label else ""
    # Short blocks stay on one page; a long one may split, rather than leave
    # most of a page empty by jumping whole to the next.
    long = may_split and code.count("\n") >= LONG_BLOCK_LINES
    cls = "codewrap long" if long else "codewrap"
    return f'<div class="{cls}">{hd}<div class="codeblock">{highlight(code, lang)}</div></div>'


STRING_OR_CHAR = re.compile(r'"(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)\'')


def code_part(line: str) -> str:
    """The line with string and char literals blanked and any // comment removed,
    so braces inside "{name}" format strings are not counted."""
    line = STRING_OR_CHAR.sub('""', line)
    cut = line.find("//")
    return line if cut < 0 else line[:cut]


def excerpt(path: str, spec: str, after: int) -> tuple[int, int, list[str]]:
    """Return (first, last, lines) for one anchor spec, searching from line `after`.

    "A"       -> the line containing A, plus the rest of its { } block or statement.
    "A ..> B" -> from the line containing A through the line containing B, then
                 on until any brackets opened in that range are closed.
    Comment and #[attribute] lines directly above the first line are included.
    """
    lines = (ROOT / path).read_text().splitlines()
    a, _, b = (x.strip() for x in spec.partition("..>"))

    def find(text: str, frm: int) -> int:
        for i in range(frm, len(lines)):
            if text in lines[i]:
                return i
        sys.exit(f"anchor not found: {path} :: {text}")

    start = find(a, after)
    first = start
    while first > 0 and lines[first - 1].strip().startswith(("//", "#[")):
        first -= 1
    stop_at = find(b, start) if b else start
    depth, opened, i = 0, False, start
    while True:
        code = code_part(lines[i])
        # Count (), [] and {} together, so a statement split over several
        # lines by rustfmt (println!(...), vec![...]) is taken whole.
        depth += sum(code.count(c) for c in "([{") - sum(code.count(c) for c in ")]}")
        opened = opened or depth > 0
        done = i >= stop_at and depth <= 0 and (
            opened or code.rstrip().endswith((";", "}")) or bool(b))
        if done:
            nxt = next((ln.strip() for ln in lines[i + 1:] if ln.strip()), "")
            if not (opened and nxt.startswith("else")):
                break
        i += 1
    return first, i, lines[first:i + 1]


def enclosing_fn(path: str, first: int, indent: int) -> str | None:
    """Name of the function whose body holds line `first` (body indented by `indent`)."""
    lines = (ROOT / path).read_text().splitlines()
    for i in range(first - 1, -1, -1):
        ln = lines[i]
        if ln.strip() and len(ln) - len(ln.lstrip()) == indent - 4 and ln.strip() != "{":
            if ln.strip().startswith(")"):
                continue  # closing line of a multi-line signature
            m = re.search(r"\bfn\s+(\w+)", ln)
            return m.group(1) if m else None
    return None


def dedent(chunk: list[str]) -> str:
    indent = min(len(ln) - len(ln.lstrip()) for ln in chunk if ln.strip())
    return "\n".join(ln[indent:] for ln in chunk)


def keep_with_next(out: list[str]) -> None:
    """A paragraph ending in ':' introduces the code below it; keep them on one page."""
    j = len(out) - 1
    while j >= 0 and not out[j].strip():
        j -= 1
    if j < 0 or not out[j].rstrip().endswith(":</p>"):
        return
    while j >= 0 and "<p>" not in out[j]:
        j -= 1
    if j >= 0:
        out[j] = out[j].replace("<p>", '<p style="break-after: avoid">', 1)


def expand(src: str) -> str:
    out, lines, i = [], src.splitlines(), 0
    while i < len(lines):
        ln = lines[i].strip()
        if ln.startswith(":::file "):
            path, *anchors = [p.strip() for p in ln[8:].split(" :: ")]
            parts, pos, prev_end, prev_ctx = [], 0, None, None
            for spec in anchors:
                first, last, chunk = excerpt(path, spec, pos)
                indent = min(len(c) - len(c.lstrip()) for c in chunk if c.strip())
                ctx = enclosing_fn(path, first, indent) if indent >= 4 else None
                if parts and first != prev_end + 1:
                    parts.append("")
                if ctx and ctx != prev_ctx:
                    parts.append(f"// in {ctx}():")
                parts.extend(dedent(chunk).split("\n"))
                pos, prev_end, prev_ctx = last + 1, last, ctx
            keep_with_next(out)
            out.append(block(dedent(parts), "rs", path))
            i += 1
        elif ln.startswith(":::include "):
            path = ln[11:].strip()
            lang = {".json": "json", ".yml": "yaml", ".yaml": "yaml", ".toml": "toml"}.get(
                Path(path).suffix, "sh")
            keep_with_next(out)
            # a whole config file reads best on one page, so it never splits
            out.append(block((ROOT / path).read_text(), lang, path, may_split=False))
            i += 1
        elif re.match(r":::(rs|py|out|sh|toml|json|yaml)\b", ln):
            lang, _, label = ln[3:].partition(" ")
            body = []
            i += 1
            while lines[i].strip() != ":::":
                body.append(lines[i])
                i += 1
            keep_with_next(out)
            out.append(block("\n".join(body), lang, label or LABELS[lang]))
            i += 1
        else:
            out.append(lines[i])
            i += 1
    return "\n".join(out)


def main() -> None:
    from weasyprint import HTML

    html_text = expand((HERE / "book.src.html").read_text())
    (HERE / "book.html").write_text(html_text)
    target = ROOT / "docs" / "Rust-for-Python-Developers.pdf"
    HTML(str(HERE / "book.html")).write_pdf(str(target))
    print(f"wrote {target}")


if __name__ == "__main__":
    main()
