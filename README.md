# Rust for Python Developers

A crash course in Rust for someone who already knows Python. It has two parts:

- **The textbook**, [`docs/Rust-for-Python-Developers.pdf`](docs/Rust-for-Python-Developers.pdf).
  Each chapter walks through a file in this repo and shows the matching Python.
  It ends with hands-on drills and an interview quick reference.
- **The companion code**, a Cargo workspace with two packages:
  a library plus a demo program with one module per chapter, and a small web API.

Every code listing in the book comes from these source files, and every output and compiler error
shown in the book was produced by running this code.

## Quick start

```bash
# 1. Install Rust (skip this if `cargo --version` prints 1.88 or later)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"                  # or open a new terminal

# 2. Build, test, run
cargo build                                # compile everything
cargo test                                 # 12 tests
cargo run                                  # run every chapter
cargo run -- 6                             # run the demo for book chapter 6
```

## Layout

```
Cargo.toml                     the tutorial package, plus [workspace] adding api/
Cargo.lock                     exact dependency versions
src/
  main.rs                      entry point: picks which chapters to run
  lib.rs                       the library root: lists every module
  ch03_basics.rs ... ch14_concurrency.rs   one file per book chapter, same number
  ch15_orders.rs               runs the order system end to end
  orders/                      the order system: products, discounts, checkout
  money.rs                     formats amounts held as whole cents
tests/                         integration tests for BankAccount and OrderService
api/                           the tutorial-api package: an axum web API over the order system
docs/
  Rust-for-Python-Developers.pdf   the textbook
  book/                        textbook source and build script
```

## Chapter map

Demo numbers match the book's chapter numbers.

| `cargo run -- N` | File | Topic |
|---|---|---|
| 3 | `ch03_basics.rs` | Syntax, `let`/`mut`, formatting, blocks as expressions |
| 4 | `ch04_types.rs` | Number types, `String` vs `&str`, tuples, `Option` |
| 5 | `ch05_control_flow.rs` | `if` expressions, `match`, loops |
| 6 | `ch06_ownership.rs` | Ownership, moves, borrowing, the borrow rule |
| 7 | `ch07_structs.rs` | Structs, `impl`, `&self`/`&mut self`, `derive` |
| 8 | `ch08_enums.rs` | Enums with data, pattern matching |
| 9 | `ch09_errors.rs` | `Result`, `?`, custom error types, panics |
| 10 | `ch10_collections.rs` | `Vec`, slices, `HashMap`, `BTreeMap`, `HashSet` |
| 11 | `ch11_traits.rs` | Traits, generics, trait objects, operator traits |
| 12 | `ch12_iterators.rs` | Closures, iterator chains, laziness |
| 13 | `ch13_lifetimes.rs` | Lifetimes, `Drop`, `Box`, `Rc`, `RefCell` |
| 14 | `ch14_concurrency.rs` | Threads, `Arc<Mutex>`, channels, async with Tokio |
| 15 | `ch15_orders.rs` + `orders/` | Everything together; dependency injection; tests |

Book chapters 16 to 18 cover the web API, the drills, and the interview quick reference.

## The web API

```bash
cargo run -p tutorial-api
```

```bash
# in a second terminal
curl localhost:3000/products
curl localhost:3000/stock/MN-03
curl -X POST localhost:3000/orders -H 'Content-Type: application/json' \
     -d '{"customer":"Ada","lines":[{"sku":"MN-03","quantity":1}]}'
```

## Practising

Chapter 17 of the book lists eight drills, from reading a failing test to adding an API endpoint.
Appendix A has the solutions. Work on a branch so you can compare or reset:

```bash
git switch -c drills        # do the drills here
```

```bash
git checkout -- .           # throw away changes to tracked files
```

```bash
git switch main             # back to the clean version
```

## Rebuilding the textbook

The PDF is generated from `docs/book/book.src.html`. `build.py` copies code excerpts out of the
source files, so the book always shows the current code. Rebuilding needs
[uv](https://docs.astral.sh/uv/) and the WeasyPrint system libraries (pango, cairo).

```bash
uv run --no-project --with weasyprint python docs/book/build.py
```

## Python to Rust cheat sheet

| Python | Rust |
|---|---|
| `f"{x:.2f}"` | `format!("{x:.2}")` |
| `list`, `dict`, `set` | `Vec<T>`, `HashMap<K, V>`, `HashSet<T>` |
| `None` / `Optional[int]` | `Option<i32>`: `Some(5)` or `None` |
| `raise` / `try: ... except` | return `Err(e)` / `match` on a `Result`, or `?` |
| `@dataclass` | `struct` with `#[derive(Debug, Clone, PartialEq)]` |
| `class Base(ABC)` / `Protocol` | `trait` |
| `[f(x) for x in xs if p(x)]` | `xs.iter().filter(p).map(f).collect()` |
| `with open(...) as f:` | the value is closed when it goes out of scope (`Drop`) |
| `threading.Lock` | `Mutex<T>` (the lock owns the data) |
| `async def` / `asyncio.gather` | `async fn` / `tokio::join!` |
| `pytest` / `assert a == b` | `cargo test` / `assert_eq!(a, b)` |
| `pip` / PyPI | `cargo add` / crates.io |
| `ruff check` / `ruff format` | `cargo clippy` / `cargo fmt` |
