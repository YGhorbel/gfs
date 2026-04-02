---
name: RFC 008 - Remote clone
title: Cloning from a remote existing database
status: Draft
---

## Remote clone using snapshots:
-- RDS, GCP, etc.

We can snapshot and then get the snapshot locally and init the database


## Remote Proxy
Reads go to the remote
Writes go the local

Results are merged

First possiblity: postgres_fdw, Foreign Data Wrapper



### Local divergence layer over a remote database
The specific combination "divergent replica query merging" or "authoritative overlay federation" has no clean paper behind it. The novel parts are:
- Authoritative precedence: local always wins over upstream for the same primary key, including tombstones for deletes
- Delta-aware aggregate rewriting: COUNT(*) must be computed as upstream_count - deleted_locally + inserted_locally, which requires tracking the shape of local mutations, not just their content
- Consistency semantics: what isolation guarantees can you even offer when one side is RDS (with its own transaction log) and the other is local?

### CLI

gfs clone --upstream rds://... --mask auto