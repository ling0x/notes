---
title: Thread
---

### Question: What exactly is a thread?

A thread is the smallest unit of execution that a CPU can run — essentially, an
independent sequence of instructions within a program. The CPU distinguishes
threads by their saved state and ID, not by “feeling” them electrically 😜.

#### Every thread has a small bundle of CPU-related data called its context:

- Program counter (which instruction to run next)

- CPU registers (temporary values)

- Stack pointer (where its stack lives)

- A thread ID the OS uses to refer to it

This context is stored in memory in a per-thread data structure managed by the
OS (often called a TCB, “thread control block”).

#### When the OS scheduler decides to run a different thread:

- It takes the context of that thread (from memory).

- It loads that context into the CPU’s registers and program counter.

- From the CPU’s perspective, it is now executing that thread.

So “which thread am I running?” = “which context did the OS most recently load
into my registers and program counter?”

### Question: Why This Matters for Arc?

This is exactly why Arc exists in Rust — when multiple threads share the same
data, they all access the same memory. Without atomic operations on the
reference count, two threads incrementing it simultaneously could corrupt it,
leading to use-after-free bugs or memory leaks. Arc's atomic reference counting
prevents this without needing a lock.

### Question: What is the relationship between a thread and a process and a program?

```
             ,----------------,              ,---------,
        ,-----------------------,          ,"        ,"|
      ,"                      ,"|        ,"        ,"  |
     +-----------------------+  |      ,"        ,"    |
     |  .-----------------.  |  |     +---------+      |
     |  |                 |  |  |     | -==----'|      |
     |  |  I LOVE DOS!    |  |  |     |         |      |
     |  |  Bad command or |  |  |/----|`---=    |      |
     |  |  C:\>_          |  |  |   ,/|==== ooo |      ;
     |  |                 |  |  |  // |(((( [33]|    ,"
     |  `-----------------'  |," .;'| |((((     |  ,"
     +-----------------------+  ;;  | |         |,"
        /_)______________(_/  //'   | +---------+
   ___________________________/___  `,
  /  oooooooooooooooo  .o.  oooo /,   \,"-----------
 / ==ooooooooooooooo==.o.  ooo= //   ,`\--{)B     ,"
/_==__==========__==_ooo__ooo=_/'   /___________,"
`-----------------------------'
               v
+--------------------------------------------------------------+
|                          PROGRAM                             |
|                 (code on disk, not running)                  |
+--------------------------------------------------------------+
        |                                     |
        | OS loads program into memory, twice |
        v                                     v
+----------------------+        +----------------------+
|      PROCESS 1       |        |      PROCESS 2       |
|  (instance of prog)  |        |  (instance of prog)  |
|  Own address space   |        |  Own address space   |
+----------------------+        +----------------------+
    |   |   |                           |   |   |
    |   |   +---------------+           |   |   +---------------+
    |   +-------+           |           |   +-------+           |
    v           v           v           v           v           v
+--------+  +--------+  +--------+  +--------+  +--------+  +--------+
|THREAD 1|  |THREAD 2|  |THREAD 3|  |THREAD A|  |TRHEAD B|  |THREAD C|
+--------+  +--------+  +--------+  +--------+  +--------+  +--------+

- PROGRAM: one set of instructions on disk
- PROCESS 1 and PROCESS 2: two running instances of the same program
- THREADs: multiple execution flows inside PROCESS 1 (PROCESS 2 could also have threads)
```

The three concepts form a clear hierarchy: a program becomes a process when run,
and a process contains one or more threads.

Program → Process → Thread

Think of it like this:

- A program is a static set of instructions stored on disk — it's just a file,
  doing nothing ​

- A process is what a program becomes when the OS loads it into memory and
  starts executing it — it's a living, running instance of the program ​

- A thread is the actual unit of execution inside a process — the sequence of
  instructions the CPU is actively running

### Question: Are threads allocated automatically to a process? How do I know how many threads a process runs? And how do I know how many process a program runs?

Every process automatically gets exactly one thread when it starts — the main
thread. This is the thread that begins executing at main(). Any additional
threads beyond that must be explicitly created by the program itself — the OS
does not add more threads on its own. So the number of threads a process has is
entirely determined by what the programmer coded.

There's no fixed number as to how many threads can a process run — it depends on
system resources. On Linux for example, the maximum is calculated as

```text
max threads = virtual memory size ÷ (stack size × 1024 × 1024)
```

In practice, a Linux system can support tens of thousands of threads (e.g.
~63,704 on a typical kernel). On Windows, the limit is also very high and
practically constrained by available memory rather than a hard cap.

A program typically runs as one process by default. It can spawn additional
processes explicitly in code (e.g. using fork() in Unix or spawning subprocesses
in Rust/Python) — but this is always a deliberate programmer choice, not
automatic.

## How to Check in Practice

On Linux/macOS (terminal):

- See threads for a specific process:

```bash
ps -o nlwp <pid>      # shows number of threads
cat /proc/<pid>/status | grep Threads
```

- See all processes from a program:

```bash
ps aux | grep <program_name>
```

- Live view with threads: run top, then press H to toggle thread view ​
