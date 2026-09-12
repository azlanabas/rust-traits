# keywords_demo

A single `main.rs` that uses all 39 of Rust's active keywords at least once,
organized around the 10 language traits that make Rust distinctive. Built as
a study aid — read the file top to bottom rather than treating it as
production code.

## Running it

```bash
cargo run
```

Requires network access on first build so Cargo can fetch `tokio` (used for
the `async`/`await` section). Everything else is standard library only.

## The 10 traits

| # | Trait | Where to look |
|---|-------|----------------|
| T1 | Ownership & the borrow checker | `&self`, `Self`, the `move` closure in `spawn_worker` |
| T2 | Expression-oriented design | `grade_for`'s `if`/`else` returning a value |
| T3 | Zero-cost abstractions | the `.iter().map().sum()` chain in `main` |
| T4 | Traits as flexible interfaces | the `Shape` trait + `Circle`/`Square` impls |
| T5 | Fearless concurrency | `thread::spawn` and the `async fn` / `.await` calls |
| T6 | Pattern matching | `match` in `find_by_status` and the `Display` impl |
| T7 | Option/Result instead of null | `find_by_status` and `parse_radius` return types |
| T8 | Macros | `trait_label!` |
| T9 | Immutability by default | `const`/`static`, contrasted with the `mut` bindings |
| T10 | The `?` operator | inside `parse_radius` |

Cargo/tooling was discussed as an 11th trait but deliberately left out — it's
an ecosystem property, not something expressible inside a source file.

## The 39 keywords

`as` `async` `await` `break` `const` `continue` `crate` `dyn` `else` `enum`
`extern` `false` `fn` `for` `if` `impl` `in` `let` `loop` `match` `mod`
`move` `mut` `pub` `ref` `return` `self` `Self` `static` `struct` `super`
`trait` `true` `type` `unsafe` `use` `where` `while` `_`

Each appears in `main.rs` with an inline `[keyword]` comment marking where,
plus a `[T#]` comment marking which trait it's demonstrating.

## Notes

- The `extern "C" { fn abs(...) }` block links directly against libc — the
  same example used in the official Rust Book's FFI chapter. It should
  compile cleanly on Linux without extra configuration.
- Built without a local Rust toolchain to test against (sandbox network
  restrictions blocked `rustup`/`apt`), so give it a run and flag anything
  that doesn't compile clean.
