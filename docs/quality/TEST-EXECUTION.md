# PdfCraft fork — repeatable test execution

**Fork:** `Rwanbt/pdfcraft` · **PR base:** `main` · **Working branch:** `test/fork-qa-decoder-baseline`

## Repository instructions

Read `AGENTS.md`, `CLAUDE.md`, the relevant crate README and `docs/gaps.md`. Use `Cargo.lock`; never rewrite upstream or merge to `main` automatically. Test data must be synthetic or explicitly licensed. The `plan/` directory referenced in `CLAUDE.md` is gitignored and not part of this checkout.

## Focused local/CI commands

```bash
cargo fmt --all -- --check
cargo clippy --locked -p pdfcraft-filters --all-targets -- -D warnings
cargo test --locked -p pdfcraft-filters --test decoder_contracts -- --test-threads=1
cargo test --locked -p pdfcraft-filters --test decoder_contracts -- --test-threads=1
cargo test --locked -p pdfcraft-filters --test decoder_contracts -- --test-threads=1
mkdir -p coverage
cargo llvm-cov -p pdfcraft-filters --locked --lcov --output-path coverage/filters.lcov
cargo llvm-cov report --summary-only
```

`cargo llvm-cov` requires the installed cargo-llvm-cov tool and Rust's `llvm-tools-preview` component. `coverage/filters.lcov` is a **crate-scoped** instrumented result, not workspace coverage. No arbitrary passing threshold is set until baseline is measured. Preserve job logs, artifact and SHA.

## Other mandatory gates (already declared in repository CI)

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask layers
cargo xtask assets
cargo xtask wasm
```

Desktop, web, Windows, macOS, FreeBSD, perf and packaging need their own applicable runtime and runner evidence. Static WASM checking is not browser E2E. Native unit tests are not an installed app smoke test.

## Actual executions at document creation

**None confirmed** for this fork, including this branch. The execution environment reading the repo did not have `cargo` or network access for cloning. GitHub Actions is the planned runner; if it does not start, this remains BLOCKED rather than passed.

Update this document with `run_id`, SHA, platform, date, commands, test runner counts and coverage artifact only **after** jobs have run.
