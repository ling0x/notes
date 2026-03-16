---
title: Connection Pool
---

A connection pool is a component (in your app or as a proxy like
PgBouncer/Pgpool-II) that keeps a limited number of database connections open
and reuses them across many client requests.

## Key ideas:

- Opening a [PostgreSQL connection](/operating_systems/postgresql_connection.md) is
  relatively expensive and each connection consumes memory; creating hundreds or
  thousands on demand is wasteful.

- The pool maintains, say, 20–50 actual DB connections and lets many more
  logical clients "borrow" them for short periods.

- When a request finishes, the connection is returned to the pool instead of
  being closed, so the next request can reuse it without the startup cost.

- The pool enforces an upper bound on concurrent PostgreSQL connections, which
  indirectly bounds per-connection memory usage.

### Question: what if the pool is full?

#### General behavior when pool is full:

- The pool has a max number of physical PostgreSQL connections it will open.

- When all of them are checked out and a new request comes in, most pool
  implementations put the request in a queue and wait for a connection to be
  returned. If no connection becomes free before a timeout, they raise an
  error/exception (often something like "pool exhausted" or "timeout waiting for
  connection").

- This is separate from PostgreSQL's own max_connections; the pool will usually
  hit its own limit first and throttle clients, which is the goal.

### Question: What if the DB itself is at max connections?

- PostgreSQL has a server-side max_connections limit; when that is reached, any
  new physical connection attempt fails with "too many connections".

- If your pool tries to open more physical connections beyond what the server
  allows (e.g. under load or after restart), those attempts will fail and bubble
  up as errors to the application (connection failure rather than "queued").
