# Fork automation/MCP test gate

**Scope:** `crates/automation` only; base `main` at `bbbe60015c571ebfc39708ee07777b89e267122a`.
**State:** pending exact-commit GitHub Actions evidence.

## Observed protocol gap

The line-oriented MCP handler had already implemented JSON-RPC 2.0 replies, error codes and valid notifications. But it did not verify `jsonrpc == "2.0"`, allowed arbitrary `id` types, and swallowed methodless messages without an `id`. This can produce invalid protocol behavior or allow an invalid envelope to execute a normal method.

## Deterministic tests

`crates/automation/tests/mcp_protocol.rs` exercises, without a PDF or network dependency:

- Wrong/missing JSON-RPC version (including a request disguised as a notification) returns `-32600`.
- Invalid identifiers (boolean, list, map, fractional number) return `-32600` with null reply identifier.
- Missing/non-string `method` returns `-32600` even without a request identifier.
- Legal integer, string and null identifiers remain unchanged; valid notifications have no response; unknown methods remain `-32601`.
- A multi-message stdio stream recovers after malformed JSON and invalid envelopes without losing subsequent requests.

The implementation change checks the outer protocol envelope *before* processing the method, while keeping existing valid MCP behavior unchanged.

## Evidence protocol

Workflow `.github/workflows/fork-quality-automation.yml` performs:
1. Workspace rustfmt and automation Clippy.
2. Three independent repetitions of the new protocol contracts.
3. Root-confinement hostile-argument regression tests with the MCP feature disabled.
4. Full automation crate tests with LLVM instrumentation and an `automation.lcov` artifact.

Do not claim this is browser or desktop UI E2E, or that all automation tools have complete branch coverage. Audit any failures, record exact SHA, and keep PR draft until all applicable gates pass.
