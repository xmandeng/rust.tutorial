# rust.tutorial

Rust study material for a Python developer: a Cargo workspace plus a textbook PDF generated from it.
The reader is new to Rust. Favour plain, conventional Rust over clever code; the one deliberate piece of
2024-edition syntax (`if let ... &&` in `src/ch09_errors.rs`) is called out in the book.

## Commands

- Build: `cargo build --workspace` (must stay at zero warnings)
- Lint: `cargo clippy --workspace --all-targets -- -D warnings`
- Format: `cargo fmt --all` (the book quotes formatted code, so format before rebuilding it)
- Test: `cargo test --workspace`
- Run chapters: `cargo run -- <n>`
- Run API: `cargo run -p tutorial-api` (listens on 127.0.0.1:3000)
- Rebuild book: `uv run --no-project --with weasyprint python docs/book/build.py`

## Rules

- The book (`docs/book/book.src.html`) pulls code with `:::file path :: anchor` directives (separator is
  ` :: ` with spaces). Renaming or rewording a line used as an anchor breaks the build, and an anchor is a
  substring match, so a new item whose name starts with an existing anchor can steal it. After any source
  change, rebuild the book and check that its stated outputs still match `cargo run`.
- Output blocks (`:::out`) in the book are copied from real runs, including compiler errors with their
  line numbers. Re-run and update them when code changes.
- Chapter 17 drills and appendix A solutions are verified against the code. Keep them in sync.
- The textbook PDF is committed so it can be read without building.
