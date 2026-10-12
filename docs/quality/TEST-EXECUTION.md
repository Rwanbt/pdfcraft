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

## First confirmed GitHub Actions execution

**Date:** 2026-10-12 · **Evidence SHA:** `0721b79c91db4e24602d1290063ecccd50e7c43e`

- **Run:** https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045 — scoped `pdfcraft-filters` quality job, Linux Ubuntu, **success**.
- Formatting and `cargo clippy --locked -p pdfcraft-filters --all-targets -- -D warnings`: **passed**.
- The five fixed-byte contracts: **5 passed, 0 failed**, repeated **3 times** with the same assertions.
- Instrumented tests: **48 unit + 5 decoder contracts + 9 property/integration = 62 passed, 0 failed**.
- Artifact: `pdfcraft-filters-lcov` (GitHub id `11755926825`); line coverage **99.61%**, function coverage **100.00%**, LLVM regions **98.49%**, branch coverage **not measured**. Exact gaps in `TEST-COVERAGE-BASELINE.md`.

A subsequent fixed-vector ASCII85 overflow regression has been added. Its execution and resulting coverage must be checked at the newer head SHA before promoting it to verified. Main cross-platform CI and packaging must be checked separately. Neither a cargo fmt pass nor a crate-specific test proves full application readiness.

The current ChatGPT container cannot run Cargo locally; these conclusions come from actual GitHub Actions job logs and LCOV, not a simulated runner.
