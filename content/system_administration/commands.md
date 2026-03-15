---
title: Linux Commands
---

# Docker Related Commands

```bash
docker ps \
--format '{{.Names | printf "%-20s"}} {{.Ports | printf "%-30s"}} {{.RunningFor}}' \
| sort
```

```bash
docker ps --format '{{.Names | printf "%-20s"}} {{.Ports}}' | sort
```

```bash
docker ps -a --format '{{.Names}}: {{.Ports}}'
```

### Question: What happens if I am on a server which ssh into its own server again?

Nothing special or dangerous happens by itself; you just get another shell on
the same machine.

Nothing special or dangerous happens by itself; you just get another shell on
the same machine

You now have two nested sessions:

- outer: your original SSH (or local) shell

- inner: the SSH session created from the server to itself.

Processes in the inner session are separate from the outer one, with their own
TTY, environment, history, etc.
