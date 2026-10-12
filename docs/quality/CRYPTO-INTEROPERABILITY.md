# Crypto / COS deterministic interoperability gate (fork)

**Repository:** `Rwanbt/pdfcraft` · **Branch:** `test/fork-qa-crypto-cos-interop`  
**Base:** `main` at `bbbe60015c571ebfc39708ee07777b89e267122a` · **Status:** pending measured CI verification

## Motivation and upstream context

The former Acrobat encrypted-catalog incompatibility ([issue #774](https://github.com/storytold/pdfcraft/issues/774)) was fixed upstream by commit [010b6778](https://github.com/storytold/pdfcraft/commit/010b677829712956448c179a18f34033a6b3aaf9), which is already an ancestor of this fork's `main`. The objective is **regression prevention**, not to rewrite that fix.

Existing `crates/cos/tests/encryption.rs::qpdf_agrees` skips when qpdf is absent. In the dedicated fork CI the independent reader must be available and running, or the job must fail.

## Test contracts added

| Domain | Assertion | Oracle |
| --- | --- | --- |
| PDF serialization | Encrypted `/Root` is a standalone `InFile` xref entry | `pdfcraft-cos` reopened xref |
| PDF security | `/Encrypt` dictionary remains standalone | `pdfcraft-cos` reopened xref |
| Object stream coverage | Packed output contains an actual `InStream` entry | Reopened xref |
| Cross-reader | RC4-40, RC4-128, AES-128 and AES-256, user and owner password, with and without object streams, open and pass `qpdf --check` | **External qpdf executable** |
| Save reproducibility | Repeated full encrypted writes with fixed seed, paper date and packing choice are byte-identical | Byte-level equality |
| AES-256 malformed dictionaries | Valid R6 credentials cannot authenticate when mandatory `/U`, `/O`, `/UE` or `/OE` values are truncated | Expected `CryptError::Malformed` |

No proprietary or non-licensed document fixtures are committed: `fixture()` creates a tiny PDF with synthetic text. All passwords are test-only string literals.

## CI evidence

Workflow: `.github/workflows/fork-quality-security.yml`, triggered by PR or manual dispatch.

1. Install `qpdf` on Ubuntu, print its version and set `PDFCRAFT_REQUIRE_QPDF=1`.
2. Check `cargo fmt` and Clippy for `pdfcraft-crypt` and `pdfcraft-cos`.
3. Execute all `pdfcraft-crypt` tests and COS encryption integration tests.
4. Run LLVM code-coverage instrumentation for both crates. Archive `coverage/crypt-cos.lcov`.

Record the SHA, exact workflow run URL, totals and uncovered source lines **after** the CI run completes; do not infer them from the presence of this workflow.

## Known limits / integration criteria

qpdf is independent of the PdfCraft writer and reader, but is **not Acrobat itself**, so this gate cannot certify every Acrobat interoperability corner case. Its `--check` validates PDF structure and decryption, not visual rendering correctness. Browser/desktop E2E, user-visible security dialogs, nondeterministic timing and resource limits require separate gates. The entire workspace's coverage cannot be inferred from these two crates.

Keep the pull request **DRAFT**. Do not merge to `main` or open an upstream PR until full relevant CI and review have succeeded.
