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
