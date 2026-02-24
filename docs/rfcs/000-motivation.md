# RFC 000 - Motivation

## Overview

Git for Database Systems (GFS) aims to enable AI agents to manage databases in an efficient and secure way.

Like Git for code, GFS will provide branching, lineage, and time travel, applied to databases.

## Design Principles

**Database-agnostic.** We want to remain agnostic to the underlying database engine and better handle storage and compute orchestration, all while keeping the database source code unmodified. Instead, we will manipulate database configurations to adapt their behavior to support branching and time travel. A pluggable provider registry will supply database-specific behavior (connection strings, versions, snapshot preparation) for PostgreSQL, MySQL, and others.

**Familiar interface.** We want to spare developers -- or anyone using GFS -- the effort of learning a new API. We find Git to be the best inspiration for that, and will build a Git-like CLI tailored to our use cases. Commands like `init`, `commit`, `checkout`, `log`, `status`, and `branch` will mirror Git's mental model so that the learning curve is minimal.

**Isolation and safety.** Each branch will get its own persistent workspace with an independent database instance running in a containerized environment. Copy-on-write snapshots will keep disk usage low while ensuring that changes on one branch never leak into another. During commits, the database container will be paused to guarantee a consistent snapshot.

**Auditability and lineage.** Every commit will capture full metadata: author, timestamps, database provider and version, and file-level change tracking (added, deleted, modified). All objects will be content-addressed with SHA-256 hashes, forming a directed acyclic graph that enables complete lineage tracing and time travel to any prior state.

**Composability and modularity.** The architecture will follow a hexagonal (ports and adapters) pattern. The domain layer will define four core ports -- Repository, Storage, Compute, and DatabaseProvider -- each with swappable adapter implementations (e.g., APFS or Btrfs for storage, Docker for compute). Use cases will orchestrate these ports without coupling to any specific implementation.

**Lightweight and minimal overhead.** Operations will be stateless and request-driven with no persistent background services. Copy-on-write snapshots will consume no extra disk space until modified. The core domain will depend on a minimal set of libraries to keep the footprint small.

**Robust implementation.** We will use Rust as the primary language for its memory safety guarantees, strong type system, and performance characteristics. Errors will be modeled as explicit types propagated through `Result`, never panicking in the hot path. All I/O will be async-first, ensuring concurrency without sacrificing safety.

**Agent-first, developer-friendly.** GFS will be designed so that both human developers and AI agents can use it effectively. The CLI will be intuitive for developers, and an MCP (Model Context Protocol) adapter will expose GFS operations as tools that AI agents can invoke directly. Developers enjoy using GFS; agents will too.

## Why AI Agents Need This

We are convinced that AI agents need a reliable versioning system for databases. GFS will enable agents to experiment, make mistakes, roll back, and retry, all without altering production databases.
