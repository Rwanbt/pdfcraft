# PdfCraft fork quality — initial coverage baseline

**Recorded:** 2026-10-12 · **Repository:** `Rwanbt/pdfcraft` · **Base:** `main` at `bbbe60015c571ebfc39708ee07777b89e267122a`  
**Work branch:** `test/fork-qa-decoder-baseline` · **Status:** UNVERIFIED / measurement pending

## Repository reconnaissance (GitHub API, not a Cargo test run)

- 30 Rust crates under `crates/`, three apps under `apps/`, and `xtask`; vendor dependencies excluded.
- 352 non-vendor `.rs` files, 85 Rust files directly under a `tests/` directory (including platform-conditional tests). This is **not** a test-count or coverage measurement.
- Existing workflows: `ci.yml`, `freebsd.yml`, `packaging-lint.yml`, `release.yml`, `windows-arm64.yml`.
- Existing CI declares Linux/Windows clippy and tests, WASM checking, dependency audit, plus scheduled fuzzing. Some native platform builds run in other workflows.
- Before this branch, the fork GitHub Actions run listing returned `total_count: 0`. Existing workflow definitions are **not** proof of passing runs.
- Upstream `storytold/pdfcraft:main` was 223 commits ahead of the fork's baseline. No sync or merge was performed.

## Measurement

| Scope | Lines | Functions | Branches | Collection |
| --- | --- | --- | --- | --- |
| `pdfcraft-filters` | Not measured | Not measured | Not measured | Pending Actions run |
| Whole workspace | Not measured | Not measured | Not measured | Pending independent instrumentation |
| Desktop/web UI | Not measured | Not measured | Not measured | Pending platform-specific tests |

There is **no defensible pre-change numeric baseline** yet. Do not treat source-file inventory or green unit tests as coverage. The scoped workflow `.github/workflows/fork-quality-filters.yml` generates a downloadable `coverage/filters.lcov` on successful execution. The 100% target is aspirational, not enforced.

## Candidate first change

Five deterministic, fixed-byte decoder contracts in `crates/filters/tests/decoder_contracts.rs` exercise:
1. Output-size limit through a *second* expanding filter, including exact boundary.
2. Strict failure versus tolerant partial output for truncated RLE.
3. Partial output through two corrupt filter stages.
4. Limit errors that must never be downgraded to recoverable corruption.
5. Zero-byte output limit.

These are **added**, not marked passed. Current environment cannot execute Cargo; GitHub Actions is the validation path. Run `cargo test --locked -p pdfcraft-filters --test decoder_contracts` and read the run for the exact PR head SHA.

## Next baseline steps

1. Obtain an actual green/red GitHub Actions result for the exact PR commit; investigate any no-run state.
2. Download `filters.lcov`, derive a per-file line baseline and list uncovered functions/branches separately.
3. Instrument isolated crates in phases, then whole workspace with compatible features; document system limitations.
4. Count collected tests from `cargo test --workspace -- --list` (account for separate binaries), not filenames.
5. Set non-regression gates only **after** a measured, reviewable baseline exists.
