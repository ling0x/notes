---
title: Busy Loop
---

### Question: 什么是 Busy Loop（忙等待循环）？

Busy loop（忙循环），也叫 busy waiting（忙等待） 或
spinning（自旋），指的是程序在等待某个条件满足时，不断地反复检查这个条件，而不是暂停让出
CPU 资源 。 ​

#### 通俗理解

想象你在等外卖，有两种等法：

- Busy loop（忙等）：你每隔一秒就跑去门口看一眼，"到了没？到了没？到了没？" —
  你一直在消耗精力，什么别的事也做不了。

- 正常等待（阻塞/回调）：你去做别的事，门铃响了再去开门 — 不浪费任何精力。

#### 你代码里的问题

帖子中的代码就是典型的 busy loop ： ​

```rust
let my_val = loop {
    if let Some(val) = shared.try_lock() {
        break val;
    }
    // 没拿到锁？继续循环，反复尝试...
};
```

这段代码在拿不到锁时会不停地循环尝试，持续占用 CPU，毫无效率
。[Busy Waiting](https://www.geeksforgeeks.org/operating-systems/busy-waiting-in-os/)

关键原则：在 Rust 异步程序中，只要你不在持有锁的情况下调用 .await，用
std::sync::Mutex 是完全安全且高效的，因为 Rust 的异步运行时只会在 .await
点才切换任务 。
