# PdfCraft quality traceability — starting matrix

**As of:** 2026-10-12 · **Baseline:** `bbbe60015c571ebfc39708ee07777b89e267122a`  
Status `UNVERIFIED` means code or tests may exist, but no passing run on this fork was confirmed. `PARTIAL` is reserved for a tested subset with identified gaps; `VERIFIED` requires observed runtime and contract assertions.

| ID | Area / expected contract | Existing evidence in tree | Added regression / verification | Status |
| --- | --- | --- | --- | --- |
| PDF-FLT-001 | ASCIIHex decoded bytes / malformed input | `crates/filters/src/asciihex.rs` unit tests | Fixed-byte zero bound and corruption cases; [run 38268331045](https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045) | PARTIAL |
| PDF-FLT-002 | RunLength exact output bounds, truncation and EOD | `crates/filters/src/runlength.rs` unit tests | Chained expansion and truncated-literal vectors; [run 38268331045](https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045) | PARTIAL |
| PDF-FLT-003 | Strict and tolerant chain result semantics | `crates/filters/src/lib.rs`; property tests | Two-stage partial-recovery vector; [run 38268331045](https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045) | PARTIAL |
| PDF-FLT-004 | Flate/LZW and predictors resist malformed inputs | `crates/filters/tests/proptests.rs` | Property and unit tests passed in [run 38268331045](https://github.com/Rwanbt/pdfcraft/actions/runs/38268331045), independent corpus still pending | PARTIAL |
| PDF-COS-001 | Object graph and incremental/full persistence fidelity | `crates/cos/tests/` | Read/write oracle and file corruption matrix pending | UNVERIFIED |
| PDF-SEC-001 | Every automation path honors `--root` | `crates/automation/tests/hostile_args.rs`; CLI `run_root.rs` | CLI and MCP cross-platform regression run needed | UNVERIFIED |
| PDF-ENC-001 | PDFs protected here open in independent readers | Existing encryption tests; upstream issue #774 | Independent encrypted-PDF interoperability acceptance test missing | UNVERIFIED |
| PDF-UI-001 | Open-edit-undo-save-reopen-export through real UI | `crates/ui-egui/tests/` (multiple tests); platform smoke tests | Native GUI E2E with binary required | UNVERIFIED |
| PDF-WEB-001 | Browser UI interactions work with WASM build | `apps/pdfcraft-web`; WASM check workflow | Real browser E2E missing | UNVERIFIED |
| PDF-REL-001 | Packaging and install/uninstall across platforms | `packaging/`; release workflows | Runner evidence for actual artifacts pending | UNVERIFIED |

The matrix is a reconnaissance **starting point**, not a complete inventory of hundreds of public tool and UI contracts. Expand by crate API, command, route and user interaction. Attach specific SHA, workflow job URL, observed assertions and instrumented file/branch coverage before promoting each entry to VERIFIED.
