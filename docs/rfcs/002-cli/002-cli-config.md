# RFC 006 — Data-plane config (CLI)

## Overview

The data-plane **config** command lets you read and write repository-level configuration, similar to `git config`. The main use is configuring **user.name** and **user.email**, which are stored in `.gfs/config.toml` and used as the default author and author email for every commit (see [006-cli-commit](006-cli-commit.md)).

This RFC defines the **command interface**, **behaviour**, and **storage** of `gfs config`.

---

## Command interface

### Get

```
gfs config user.name
gfs config user.email
```

### Set

```
gfs config user.name "<name>"
gfs config user.email "<email>"
```

### Optional path

```
gfs config [--path <dir>] <key> [<value>]
```

| Flag | Required | Description |
|------|----------|-------------|
| `--path` | no | Path to the GFS repository root (directory that contains `.gfs/`). Defaults to the current working directory. |
| `key` | **yes** | Configuration key. Supported: `user.name`, `user.email`. |
| `value` | for set | Value to write. Omit to read. |

Semantics match git: one argument after the key means **get**; two arguments (key + value) means **set**.

---

## Behaviour

- **Get**: Read the value for the given key from `.gfs/config.toml`. If the key or the `[user]` section is missing, print nothing and exit 0. Otherwise print the value to stdout (single line, no extra output).
- **Set**: Ensure `.gfs/` and `config.toml` exist (init if needed), then update or create the `[user]` section with the given key/value. Other keys in `config.toml` (e.g. `mount_point`, `environment`, `runtime`) are preserved. Write the updated TOML back to `.gfs/config.toml`.

Only `user.name` and `user.email` are supported. Other keys (e.g. `user.other`) are out of scope and may be rejected or ignored.

---

## Storage: config.toml

Configuration is stored in **`.gfs/config.toml`** at the repository root. User identity is stored under a `[user]` section:

```toml
[user]
name = "Alice"
email = "alice@example.com"
```

- **user.name** — Stored as `user.name` in TOML (table `user`, key `name`). Used as default commit author when `--author` is not passed.
- **user.email** — Stored as `user.email` in TOML (table `user`, key `email`). Used as default commit author email when `--author-email` is not passed.

This file may also contain other sections (e.g. `mount_point`, `environment`, `runtime`). The config command must only add or update the `[user]` section and leave the rest unchanged when setting `user.name` or `user.email`.

---

## Integration with commit

Each commit records **author** and **author email** (see [006-cli-commit](006-cli-commit.md)). Those values are resolved in this order:

1. CLI flags `--author` / `--author-email` if present.
2. Otherwise `user.name` / `user.email` from `.gfs/config.toml`.
3. For name only: fallback to `"user"` if still unset; email remains unset.

So configuring `user.name` and `user.email` via `gfs config` ensures every subsequent commit (without overrides) is attributed to that identity.

---

## Output

- **Get (value present)**: Print the value, no trailing newline required (one line to stdout).
- **Get (value or key missing)**: Print nothing; exit 0.
- **Set**: No output on success; exit 0.
- **Error**: Message to stderr, non-zero exit (e.g. not a repo for get; invalid key; write failure).

---

## Examples

```sh
# Set identity (like git config user.name / user.email)
gfs config user.name "Alice"
gfs config user.email "alice@example.com"

# Read back
gfs config user.name    # → Alice
gfs config user.email   # → alice@example.com

# In a specific repo
gfs config --path /data/my-repo user.email "bob@example.com"
```

---

## Domain / adapters

- **Reading**: The repository port already exposes `get_user_config(repo) -> Option<UserConfig>`. The CLI can use the same mechanism (e.g. load `GfsConfig` from `.gfs/config.toml` and read `config.user`).
- **Writing**: The CLI (or a small use case) loads `GfsConfig`, updates or creates the `user` section, and saves via `GfsConfig::save`. No new port is required if config is considered a repo layout concern.

---

## Out of scope

- Global or system-level config (e.g. `~/.gfsconfig`); only repo-local `.gfs/config.toml` is in scope.
- Other config keys (e.g. `core.*`, `runtime.*`) may be added in a later RFC.
- Removing a key (e.g. `gfs config --unset user.email`) is not required for this RFC.
