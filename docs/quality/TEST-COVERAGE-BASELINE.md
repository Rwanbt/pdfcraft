# PdfCraft fork quality — initial instrumented baseline

**Recorded:** 2026-10-12 · **Repository:** `Rwanbt/pdfcraft` · **Base:** `main` at `bbbe60015c571ebfc39708ee07777b89e267122a`  
**Work branch:** `test/fork-qa-decoder-baseline` · **Scoped crate status:** PARTIALLY VERIFIED · **Whole app:** UNVERIFIED

## Repository reconnaissance (GitHub API)

- 30 Rust crates under `crates/`, three apps under `apps/`, and `xtask`; vendored dependencies excluded.
- 352 non-vendor `.rs` files; 85 Rust files directly under a `tests/` directory (including platform-conditional tests). These counts are **not** runtime test counts or coverage.
- Workflows on `main`: `ci.yml`, `freebsd.yml`, `packaging-lint.yml`, `release.yml`, `windows-arm64.yml`.
- Before this PR the fork returned zero Actions runs; PR #1 successfully triggered the Actions service.
- As of first audit `storytold/pdfcraft:main` was 223 commits ahead of fork `main`; no synchronization, upstream PR or merger was performed.

## First measured coverage (completed run)

**Evidence commit:** `0721b79c91db4e24602d1290063ecccd50e7c43e`  
**GitHub run:** https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045  
**LCOV artifact:** `pdfcraft-filters-lcov`, artifact id `11755926825`, captured 2026-10-12, Ubuntu Linux.

| Scope | Lines | Functions | LLVM code regions | Branches |
| --- | --- | --- | --- | --- |
| `pdfcraft-filters` | **1011/1015 = 99.61%** | **124/124 = 100.00%** | **2354/2390 = 98.49%** | **Not measured** |
| Whole workspace | Not measured | Not measured | Not measured | Not measured |
| Desktop/web UI | Not measured | Not measured | Not measured | Not measured |

**Test evidence:** `pdfcraft-filters` ran 48 unit tests + 5 fixed-vector decoder contracts + 9 property/integration tests = **62 passed, zero failures**. The five added tests also passed in three successive separate invocations. Existing lint and format steps in the scoped workflow passed for the evidence SHA. Tests assert expected decoded bytes/errors; round-trip properties are not the only oracles.

| Source | Lines | Executed | Coverage |
| --- | ---: | ---: | ---: |
| `ascii85.rs` | 116 | 115 | 99.14% |
| `asciihex.rs` | 74 | 74 | 100% |
| `flate.rs` | 105 | 103 | 98.10% |
| `lib.rs` | 183 | 182 | 99.45% |
| `lzw.rs` | 223 | 223 | 100% |
| `predictor.rs` | 229 | 229 | 100% |
| `runlength.rs` | 85 | 85 | 100% |

### Exactly four uncovered lines on the first measurement

1. `ascii85.rs:55`: overflow rejection of a short final ASCII85 group. **Follow-up regression added** on this branch; require a new Actions run before claiming its coverage.
2. `flate.rs:44`: defensive output-capacity allocation guard, likely superseded by earlier output-length checks. Verify its reachability by code/control-flow analysis before excluding it.
3. `flate.rs:146`: intentionally unexecuted `panic!` failure arm inside a `#[test]` assertion. Not a production path.
4. `lib.rs:179`: defensive unsupported-filter fallback after `run()` pre-validates and rejects every nondecodable filter. Not reachable through the public decoder API under current dispatch, but preserve it as defense in depth.

**Important:** the report is an instrumented crate measurement, not workspace coverage. Region coverage is not the same as branch coverage. The first numeric baseline is **after** adding the initial tests; a pre-change numeric baseline was not obtained. Do not misstate it as a before-versus-after improvement. The next run will validate the additional ASCII85 vector.

## Priority backlog

1. Check the exact latest PR head and investigate Linux/Windows/WASM/packaging job failures (if any).
2. Obtain a fresh LCOV after the new ASCII85 test, compare line hits, justify truly unreachable paths.
3. Measure the remaining crates independently and together with compatible features, without merging incompatible percentages.
4. Collect workspace test counts via the actual test runner, rather than the 85 `tests/` filenames.
5. Add independent third-party interoperability, actual desktop/browser E2E and installation/packaging verification.
6. Set non-regression gates based on real baselines, adding thresholds progressively rather than declaring 100% without evidence.
