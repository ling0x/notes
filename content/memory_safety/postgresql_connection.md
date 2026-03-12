---
title: PostgreSQL Connection
---

A PostgreSQL connection is the link between a client (like an app, script, or
psql) and a PostgreSQL database server that lets them send queries and receive
results.

## Memory and Processes

A PostgreSQL connection uses memory because each client gets its own backend
process with its own state (caches, buffers, variables, temp data, etc.), and
that state must live somewhere in RAM. This is also why too many connections can
exhaust memory and hurt performance.

PostgreSQL is a process‑per‑connection model (on typical
[Unix-like systems](/operating_systems/README.md) a new OS process is forked for
each client connection).

Each backend process has its own private address space for session-local state,
which avoids accidental shared mutable state between connections.

Backends share only explicit shared-memory regions and files (e.g., shared
buffer cache, WAL, locks), with well-defined synchronization, so there’s no
“mysterious” shared state like in a multi-threaded process with global
variables.

So, you do get isolation of most state per connection, with only controlled,
intentional shared memory for common data structures.
