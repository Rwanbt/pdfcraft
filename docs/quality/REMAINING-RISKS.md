# PdfCraft fork — open QA and production risks

**Assessed:** 2026-10-12 · **Base SHA:** `bbbe60015c571ebfc39708ee07777b89e267122a`

| Priority | Risk / observation | Current evidence | Required closure |
| --- | --- | --- | --- |
| P1 | Encrypted PDF interoperability is not independently re-verified on the fork | Upstream issue [#774](https://github.com/storytold/pdfcraft/issues/774) is **closed**. Fix [#836](https://github.com/storytold/pdfcraft/commit/010b677829712956448c179a18f34033a6b3aaf9) keeps an encrypted document's catalog outside object streams and **is already an ancestor of fork main**. The fork's `docs/gaps.md` has stale wording. | Add a known synthetic file and independent cross-reader acceptance test; audit other encryption modes. Do **not** claim the historical fix is missing. |
| P1 | Cross-platform CI and E2E remain to be fully checked | After a zero-run initial state, PR #1 triggered Actions. Scoped Linux filter run [38268331045](https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045) passed on SHA `0721b79c91db4e24602d1290063ecccd50e7c43e`; other platforms require checking at the latest SHA. | Complete Linux/Windows/WASM/packaging checks for current commit and add real GUI/browser E2E. |
| P1 | Coverage is not measured across the whole application or branches | Filters crate measured 99.61% lines, 100% functions, 98.49% LLVM regions at SHA `0721b79c91db4e24602d1290063ecccd50e7c43e`; **branch coverage not measured**. | Expand LCOV to other crates and independent app entry points; distinguish LLVM regions from branches. |
| P1 | Desktop and web E2E not fully evidenced | Source tests and WASM checks exist; no observed live GUI/browser run here | App launch, actual UI actions, file round-trip, browser console checks, native smoke and packaging |
| P1 | Fork lags upstream | `storytold/pdfcraft:main` ahead by 223 commits on 2026-10-12 | Review relevant upstream fixes without automatic merge or force-push |
| P1 | Brand/trademark asset compliance on redistributed fork | `AGENTS.md` §1.2 and README require removing ArtCraft trademark assets in forks; this checkout still contains `docs/brand` and related references | Audit permitted use; remediate references, package assets and attribution as an independent coordinated change before distributing a modified product |
| P2 | macOS excluded from primary CI matrix | `.github/workflows/ci.yml` uses Ubuntu and Windows; macOS is covered separately by release | Prove actual macOS build/smoke and decide on PR cadence |
| P2 | Property fuzzing and malformed document corpus not yet verified | Property tests exist; fuzz scheduled in YAML; no fork run evidence | Fixed seeds, downloadable findings, bounded corpus replay and regression tests |

This file describes **unresolved validation risks**, not claims of newly reproduced defects. The encrypted-catalog fix was already merged before our base SHA; what is missing is independent interoperability proof. The scoped decoder tests do not establish UI integration or whole-app production readiness.

**Integration policy:** keep PR draft, do not merge `main` or open an upstream PR until full applicable checks and a reviewable evidence trail exist.
