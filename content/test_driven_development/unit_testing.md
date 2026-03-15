---
title: Unit Testing
---

## Writing Isolated and Focused Unit Tests

Unit tests should be **isolated and focused**, testing one small, well-defined
unit of functionality at a time. Each test should verify a single behavior
without relying on other parts of the system. When tests are too broad or
tightly coupled across components, they become brittle — a minor change in one
area can cause unrelated tests to fail.

To ensure reliability and maintainability, unit tests should:

- Run independently of external systems or global state.
- Use **mocks or stubs** to replace dependencies.
- Follow the **Arrange–Act–Assert** pattern for clarity.
- Be **deterministic** and **fast** so they can run often during development.

Writing tests this way builds confidence in each unit, simplifies debugging, and
supports modular, testable code design.

---

## Enabling CI/CD Test Environments with Isolated Resources

For continuous integration workflows such as **GitHub Actions** or **Forgejo**,
unit tests should run within a fully isolated and reproducible environment. This
means the application should be capable of **spinning up a dedicated test
PostgreSQL instance** and a **test server API** that operates on a separate port
with its own **test-specific environment variables**.

These configurations ensure that test runs do not interfere with production or
staging databases. The test infrastructure should start up quickly and shut down
cleanly as part of the CI/CD pipeline, allowing automated workflows to execute
the full test suite independently for every build or pull request. This approach
guarantees repeatable, safe testing while maintaining complete separation
between test and production systems.

---

### Question: What's the difference between intergation tests and unit tests?

Unit tests focus on small, isolated pieces of code and run very fast, so they
give precise, quick feedback and make it easy to iterate or refactor without
breaking unrelated behavior. In contrast, integration tests exercise multiple
components together (like API, database, and services) to verify real workflows,
which provides higher confidence that the system behaves correctly as a whole.

The tradeoff is that integration tests are slower, more complex to set up, and
failures can be harder to diagnose, since a small change in one part of the
system can break a test somewhere else. Because of this, you typically rely on
unit tests for rapid development of specific functionality, and use integration
tests more sparingly to ensure that the integrated application still works end
to end.

---

## Abstractions and Patterns to Keep Tests Stable

This is where **interfaces** (traits), the facade and adapter patterns,
dependency injection, and internal mutability come in. When a piece of code is
wrapped behind a stable trait or adapter interface, the call sites that depend
on it can remain unchanged, while other developers are free to change the
internals of the wrapped functions without breaking those callers. This gives
more confidence in integration testing when integrating with different external
systems, call sites, or frontends, because the surface contracts stay stable
even as implementations evolve. These abstraction layers reinforce separation of
concerns between core functions, business logic, infrastructure, and UI/API
layers, so each layer can be unit-tested in isolation and then composed
predictably in integration tests.
