---
title: Design Principles
---

## Separating what changes from what stays the same

Identify the aspects of your application that vary and separate them from what
stays the same.

As simple as this concept is, it forms the basis for almost every design
pattern. All patterns provide a way to let some part of a system vary
independently of all other parts.

## Program to an interface, not an implementation.

The point is to exploit polymorphism by programming to a supertype so that the
actual runtime object isn’t locked into the code.
