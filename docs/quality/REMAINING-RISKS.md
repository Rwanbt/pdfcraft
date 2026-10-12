# PdfCraft fork — open QA and production risks

**Assessed:** 2026-10-12 · **Base SHA:** `bbbe60015c571ebfc39708ee07777b89e267122a`

| Priority | Risk / observation | Current evidence | Required closure |
| --- | --- | --- | --- |
| P0 | Cross-reader compatibility of PDFs encrypted by PdfCraft | Upstream `docs/gaps.md` ranks this #1, cites issue [#774](https://github.com/storytold/pdfcraft/issues/774) (protected PDF opens in some readers but fails in Acrobat/PDF-XChange). Not reproduced here. | Synthetic fixture, independent reader verification, minimal fix with regression |
| P0 | No CI executions confirmed on fork | Initial GitHub Actions runs API reported `total_count: 0`, despite workflows in tree | Trigger CI for current PR SHA, resolve enablement/permissions if missing, inspect every failed job |
| P1 | No coverage baseline or per-crate/function/branch metrics | No downloadable LCOV found in initial Actions history | Run instrumented workflow, archive report, measure scope before thresholds |
| P1 | Desktop and web E2E not fully evidenced | Source tests and WASM checks exist; no observed live GUI/browser run here | App launch, actual UI actions, file round-trip, browser console checks, native smoke and packaging |
| P1 | Fork lags upstream | `storytold/pdfcraft:main` ahead by 223 commits on 2026-10-12 | Review relevant upstream fixes without automatic merge or force-push |
| P1 | Brand/trademark asset compliance on redistributed fork | `AGENTS.md` §1.2 and README require removing ArtCraft trademark assets in forks; this checkout still contains `docs/brand` and related references | Audit permitted use; remediate references, package assets and attribution as an independent coordinated change before distributing a modified product |
| P2 | macOS excluded from primary CI matrix | `.github/workflows/ci.yml` uses Ubuntu and Windows; macOS is covered separately by release | Prove actual macOS build/smoke and decide on PR cadence |
| P2 | Property fuzzing and malformed document corpus not yet verified | Property tests exist; fuzz scheduled in YAML; no fork run evidence | Fixed seeds, downloadable findings, bounded corpus replay and regression tests |

This file is a list of **unresolved** risks, not an assertion that the current branch introduced them or a claim of reproduced bugs. The scoped decoder test addition does not close PDF encryption or UI integration issues.

**Integration policy:** keep PR draft, do not merge `main` or open an upstream PR until full applicable checks and a reviewable evidence trail exist.
