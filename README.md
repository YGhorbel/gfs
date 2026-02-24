# Guepard

## Testing

### Run all tests

```bash
cargo test
```

### Integration tests

Integration tests live in `integration_tests/tests/` and exercise the CLI via `cargo run`.

### E2E checkout with Postgres (`e2e_checkout_postgres`)

The `e2e_checkout_postgres` tests share a single repo and run sequentially. **You must run them with `--test-threads=1`** so they execute in order and share state:

```bash
cargo test --package integration_tests --test e2e_checkout_postgres -- --test-threads=1
```

Without `--test-threads=1`, tests run in parallel and each gets a fresh repo, causing failures such as:
- `revision not found: 'main'`
- `container should exist from test_01`
- `second commit has parent`

**Requirements:**
- macOS (commit uses APFS storage backend)
- Docker running (Postgres container)
