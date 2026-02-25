# CLI: list providers (`gfs providers`)

List database providers and their supported versions in a scrollable table. The user exits the view with **Escape** or **q**.

## Table content

| database_provider | version | features |
|-------------------|---------|----------|
| postgres | 13, 14, 15, 16, 17, 18 | tls, schema, masking, auto-scaling, performance-profile, backup, import, replication, ai-agents |
| mysql | 8.0, 8.1 | tls, schema, masking, backup, import |
| mariadb | 10.6, 10.11, 11.x | tls, schema, masking, backup, import |
| clickhouse | 24.x, 25.x | tls, schema, backup, import |

- **database_provider**: Identifier used in repo config (`environment.database_provider`).
- **version**: Supported major/minor versions; the CLI and compute adapter use these when resolving the image tag (e.g. `postgres:16`).
- **features**: Capabilities supported for that provider (e.g. TLS, schema handling, backup/restore). Exact support may depend on the compute adapter and runtime.

By default, container images for these providers are pulled from **Docker Hub** (e.g. `postgres:16`, `mysql:8.0`). Configuration may allow overriding the image registry or full image name for air-gapped or private-registry setups.

## Interaction

- Table is scrollable (vertical and horizontal if needed).
- Exit: **Escape** or **q**.
- Optional: column sorting or filter by provider/version (future enhancement).
