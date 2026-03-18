---
title: State Management
---

## Vanilla TypeScript Core State Management Patterns

The foundational approach is a typed observable store — a generic `Store<T>`
class that holds state, exposes `getState()`, and notifies subscribers via
`setState()`. TypeScript generics enforce that only valid keys/shapes can be
set. Three main patterns build on this:

- [Observable Store](/web_development/architecture/state_management/observer_pattern.md)
  — a central class with `subscribe/setState`, good for simple-to-medium apps

- [Action/Reducer](/web_development/architecture/state_management/action_reducer_pattern.md)
  — Redux-style discriminated union actions processed by a pure `reducer`
  function; TypeScript's exhaustive checking ensures no action is missed

- [Proxy-based reactivity](/web_development/architecture/state_management/proxy_based_reactivity.md)
  — intercepts property assignments to auto-trigger re-renders, mimicking
  Vue/MobX without a library

- [Path-based (DeepState)](/web_development/architecture/state_management/path_based_state.md) -
  Best for bested state trees

- [Persistent State](/web_development/architecture/state_management/persistent_state.md) -
  Best for cross-session state

  Best practices: always define a state interface, use `Partial<T>` for updates,
  prefer immutable spreads, and normalise collections as `{ byId, allIds }`.

### Difference between observer, singleton channel and BroadcastChannel

1. Observer — one object talking to its own watchers

2. [Singleton Channel](/web_development/architecture/state_management/singleton_channels.md)
   — any part of your app talking to any other part, within the same tab

3. [BroadcastChannel](/web_development/architecture/state_management/broadcastchannel_api.md)
   — any tab talking to any other tab in the same browser

In practice, you'd often use all three together: a `BroadcastChannel` receives a
cross-tab message → publishes onto a singleton `Channel<T>` → individual
components subscribed via the Observer pattern react to the update.

## TypeScript-Specific Best Practices

- Always define a state interface — never use any for your state shape ​

- Use `Partial<T>` for updates — so you only pass the fields you're changing

- Prefer immutable updates — spread operators `({ ...state, ...updates })`
  instead of mutating directly ​

- Normalize entity collections — store arrays as
  `{ byId: Record<id, T>, allIds:
  id[] }` for O(1) lookups ​

- Use `localStorage` with version migrations — persist state but include a
  `_version` field so you can safely migrate old data
