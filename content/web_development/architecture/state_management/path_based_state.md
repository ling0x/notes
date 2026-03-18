---
title: Path-Based State (Deep State)
---

```
State Tree
+----------------------------------+
|  app                             |
|    user                          |
|      profile                     |
|        name: "Alice"  <--- path  |
|        age:  30                  |
|    cart                          |
|      count: 2                    |
+----------------------------------+

subscribe("user.profile.name") --> only fires when name changes
subscribe("cart.count")        --> only fires when count changes
```

Instead of subscribing to the whole state object, subscribers listen to a
specific path within a nested state tree — e.g. "user.profile.name". Only
changes to that exact path trigger their callback.

```ts
type Path<T> = string; // e.g. "user.profile.name"

class DeepStore<T extends object> {
  private state: T;
  private subscribers = new Map<string, Set<(val: unknown) => void>>();

  constructor(initial: T) {
    this.state = initial;
  }

  subscribe<V>(path: Path<T>, cb: (val: V) => void): () => void {
    if (!this.subscribers.has(path)) this.subscribers.set(path, new Set());
    this.subscribers.get(path)!.add(cb as (val: unknown) => void);
    return () =>
      this.subscribers.get(path)?.delete(cb as (val: unknown) => void);
  }

  set(path: Path<T>, value: unknown) {
    const keys = path.split(".");
    let cursor: any = this.state;
    for (let i = 0; i < keys.length - 1; i++) cursor = cursor[keys[i]];
    cursor[keys[keys.length - 1]] = value;
    this.subscribers.get(path)?.forEach((cb) => cb(value));
  }

  get(path: Path<T>): unknown {
    return path.split(".").reduce((obj: any, key) => obj?.[key], this.state);
  }
}

// Usage
interface AppState {
  user: { profile: { name: string; age: number } };
  cart: { count: number };
}

const store = new DeepStore<AppState>({
  user: { profile: { name: "Alice", age: 30 } },
  cart: { count: 2 },
});

store.subscribe<string>(
  "user.profile.name",
  (name) => console.log(`Name changed: ${name}`),
);
store.set("user.profile.name", "Bob"); // fires callback
store.set("cart.count", 5); // does NOT fire name callback
```

Use case: Large nested state trees where you want granular, performant
subscriptions — only the parts of the UI that care about a specific slice
re-render.
