---
title: Drop
---

Rust’s Drop trait lets a type run custom cleanup code when a value is about to
go out of scope.

The drop method is called automatically when an object goes out of scope. It is
part of the std library in Rust. Box, Vec, String, File, and Process are some
examples of types that implement the Drop trait to free resources.
