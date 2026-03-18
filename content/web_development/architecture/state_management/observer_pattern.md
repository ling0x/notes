---
title: Observer Pattern
---

## Centralized Store (Observable Pattern)

```
Observer pattern (object-centric)
------------------------------------

        +-------------------+
        |    UserService    |   subject owns its observers
        +-------------------+
        |         |         |
        v         v         v
   +--------+ +--------+ +--------+
   | View A | | View B | | Logger |
   +--------+ +--------+ +--------+
```

The most fundamental approach is a typed store class that holds state, exposes a
getState() method, and notifies subscribers on change:

```ts
interface AppState {
  user: User | null;
  theme: "light" | "dark";
}

class Store<T> {
  private state: T;
  private subscribers = new Set<(state: T) => void>();

  constructor(initialState: T) {
    this.state = { ...initialState };
  }

  getState(): T {
    return { ...this.state };
  }

  setState(updates: Partial<T>) {
    this.state = { ...this.state, ...updates };
    this.subscribers.forEach((cb) => cb(this.getState()));
  }

  subscribe(cb: (state: T) => void): () => void {
    this.subscribers.add(cb);
    return () => this.subscribers.delete(cb);
  }
}
```

TypeScript generics make the store fully type-safe — setState only accepts keys
that exist on your state interface.
