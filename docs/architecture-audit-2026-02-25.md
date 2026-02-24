# GFS Architecture Audit — 2026-02-25

---

## 1. Architecture

### Pattern: Hexagonal (Ports & Adapters)

The project applies hexagonal architecture cleanly and consistently:

```
[CLI / MCP]  →  [Use Cases]  →  [Domain Ports]  →  [Adapters]
                                  Repository          APFS (macOS)
                                  StoragePort         Docker (compute)
                                  Compute             PG/MySQL/CH (providers)
                                  DatabaseProvider
```

**Strengths:**
- `gfs-domain` has zero infrastructure dependencies — pure business logic
- Ports defined as Rust traits; adapters implement them independently
- Use cases orchestrate logic without touching infra details
- CLI and MCP share the exact same use cases (no duplication)

**Weaknesses:**
- Only one storage adapter (APFS/macOS). Btrfs/ZFS are noted in docs but not implemented — limits cross-platform viability
- The in-memory `Repository` adapter is suitable for tests but there is no production filesystem-backed `Repository` adapter for the repository index (commits, refs, HEAD) — only the `.gfs` directory layout utilities exist in `repo_utils/`
- `gfs-cli` has a macOS-only dependency (`gfs-storage-apfs`), meaning the binary will not compile on Linux/Windows — no graceful degradation or conditional feature flags

---

## 2. Project Structure

### Workspace Layout (8 crates)

```
crates/
├── domain/                 # Core domain (model, ports, use cases)
├── config/                 # TOML config parsing
├── telemetry/              # Structured logging
├── applications/
│   ├── cli/                # gfs binary (Clap)
│   └── mcp/                # gfs-mcp binary (rmcp)
└── adapters/
    ├── storage-apfs/       # macOS APFS COW snapshots
    └── compute-docker/     # Docker container lifecycle
```

**Strengths:**
- Clear separation of concerns — each crate has a single responsibility
- Workspace-level dependency management avoids version drift
- Applications depend inward; adapters depend on domain ports (correct direction)
- `docs/rfcs/` provides design rationale for major components (storage, compute, MCP, CLI, disk layout)

**Weaknesses:**
- `gfs-config` and `gfs-telemetry` are thin wrappers — could be modules within another crate rather than standalone crates (minor)
- No `gfs-storage-*` crate for Linux (Btrfs/ZFS not yet implemented)
- `integration_tests` is a separate workspace member rather than using `tests/` inside relevant crates — acceptable for E2E but means test infra is disconnected from the code it tests

---

## 3. Maturity Signals

### Positive Indicators

| Area | Signal |
|------|--------|
| Language & Toolchain | Rust 2024 edition, pinned toolchain (1.93.1), MSRV enforced |
| Error Handling | `thiserror` for structured errors, `anyhow` for application-level, no broad `unwrap()` in hot paths |
| Async | Tokio throughout, consistent async/await |
| Observability | `tracing` + structured JSON logging via `gfs-telemetry` |
| CI/CD | GitHub Actions: build, test, clippy, fmt check on push and PR |
| Code Quality | `.rustfmt.toml` + `.clippy.toml` with `-D warnings` enforced in CI |
| Documentation | RFC docs for all major components |
| Release Build | LTO thin, opt-level 3, strip enabled — production-aware |
| Content Addressing | SHA-256 for commit hashes — correct for immutability guarantees |
| MCP Integration | First-class AI-agent support via rmcp — ahead of the curve |

### Negative / Immaturity Indicators

| Area | Signal |
|------|--------|
| Platform Support | macOS only (APFS). Linux path is documented but unimplemented |
| Test Coverage | Integration tests require Docker + macOS; no mock/stub layer for unit testing storage or compute |
| Persistence | In-memory `Repository` adapter suggests the core repository index (commits, refs) may not be fully persisted to disk |
| Cargo.lock | Cargo.lock is untracked — for binary projects it should be committed for reproducible builds |
| License | BUSL-1.1 — not OSS; converts to open source after a specified date. Relevant for ecosystem adoption |
| Version | No published version tag; single initial commit — very early stage |
| Kubernetes | `kube` dependency declared but no corresponding crate/adapter implemented |
| Benchmarks | No `benches/` directory — performance characteristics are untested |

---

## 4. Design Quality

### Strengths
- `.gfs/` disk layout mirrors Git's `.git/` — well-understood model, reduces cognitive overhead
- Copy-on-write snapshots (APFS `clonefile`) is the correct primitive for zero-copy branching
- Database checkpointing before snapshot (e.g., PostgreSQL `CHECKPOINT`) shows awareness of crash consistency
- Provider registry pattern allows runtime provider selection without recompilation

### Concerns
- Snapshot model is currently filesystem-based (APFS COW). Tar-based immutable bundles are documented as future work — until then, snapshots are mutable at the OS level
- No conflict resolution model documented — database branches that diverge and need merging is an unsolved problem in the current design
- MCP security model is documented as "future work" — tool exposure to AI agents without AuthN/AuthZ is a risk for production use

---

## 5. Overall Assessment

| Dimension | Rating | Notes |
|-----------|--------|-------|
| Architecture | **Strong** | Clean hexagonal design, correct dependency direction |
| Code Quality | **Good** | Idiomatic Rust, strict CI, structured errors |
| Structure | **Good** | Well-organized workspace, clear crate boundaries |
| Completeness | **Early** | macOS-only, in-memory adapters, no merge model |
| Documentation | **Good** | RFC-driven design, but no inline API docs (rustdoc) |
| Testability | **Moderate** | E2E tests exist; unit-test isolation limited by macOS/Docker deps |
| Production Readiness | **Low** | Single platform, BUSL license, no versioned release |

**Summary:** GFS has a solid architectural foundation with clean separation of concerns and good Rust practices. The design is forward-thinking (AI-first, COW snapshots, content addressing). However, it is early-stage: macOS-only, no Linux storage backend, in-memory repository index, no merge semantics, and no conflict resolution model. The RFC documentation indicates the team thinks carefully about design before coding, which is a positive signal for long-term quality.

---

## 6. Recommendations (Priority Order)

1. **Commit `Cargo.lock`** — Binary projects should always commit the lock file for reproducible builds
2. **Add a filesystem-backed `Repository` adapter** — The in-memory adapter is insufficient for production; persist commits/refs to `.gfs/objects/` and `.gfs/refs/`
3. **Abstract storage behind a runtime feature flag** — Use Rust `cfg` features or a factory to allow `gfs-cli` to compile on Linux (even without a working storage backend)
4. **Add unit test coverage for domain logic** — Use cases and domain model can be tested with the in-memory adapter without Docker or APFS
5. **Publish `rustdoc`** — The RFC docs are good; supplement with inline `///` documentation on public APIs
6. **Define the merge/conflict model** — This is the hardest unsolved problem in the design; even a "no merge" policy needs to be made explicit
