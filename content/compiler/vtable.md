---
title: Vtable
---

A **vtable** (virtual function table) in Rust is a data structure used to enable
dynamic dispatch with trait objects (`dyn Trait`). It is essentially a lookup
table of function pointers that allows the correct method implementation to be
called at runtime, even when the concrete type isn't known at compile time.
