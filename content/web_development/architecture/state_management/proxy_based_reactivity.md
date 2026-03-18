---
title: Proxy-based Reactivity
---

```
                 Proxy (trap layer)
                +------------------+
state.count++   |   set trap fires |---> notify subscribers
<-- direct -->  |   get trap fires |---> track dependencies
                +------------------+
                       |
                +------------------+
                |   Raw state obj  |
                +------------------+
```

## Reactive State with Proxy

A JavaScript Proxy wraps your state object and intercepts get and set
operations. Instead of calling setState() manually, any direct property
assignment automatically triggers subscribers — the same mechanic Vue 3 and MobX
use internally.

```ts
type Subscriber<T> = (state: T) => void;

function reactive<T extends object>(
  initial: T,
  onChange: Subscriber<T>,
): T {
  return new Proxy(initial, {
    set(target, prop, value) {
      (target as any)[prop] = value;
      onChange(target);
      return true;
    },
  });
}

// Usage
interface AppState {
  count: number;
  user: string | null;
}

const state = reactive<AppState>(
  { count: 0, user: null },
  (s) => console.log("State changed:", s),
);

state.count++; // logs automatically
state.user = "Alice"; // logs automatically
```

For finer control, you can track which properties were accessed (get trap) and
only notify subscribers that depend on that specific property — this is called
dependency tracking:

```ts
const subscribers = new Map<string | symbol, Set<() => void>>();
let activeEffect: (() => void) | null = null;

function reactive<T extends object>(initial: T): T {
  return new Proxy(initial, {
    get(target, prop) {
      if (activeEffect) {
        if (!subscribers.has(prop)) subscribers.set(prop, new Set());
        subscribers.get(prop)!.add(activeEffect);
      }
      return (target as any)[prop];
    },
    set(target, prop, value) {
      (target as any)[prop] = value;
      subscribers.get(prop)?.forEach((fn) => fn());
      return true;
    },
  });
}

function effect(fn: () => void) {
  activeEffect = fn;
  fn(); // run once to collect dependencies via get traps
  activeEffect = null;
}

// Usage
const state = reactive({ count: 0, user: "Alice" });

effect(() => {
  console.log(`Count is: ${state.count}`); // only re-runs when count changes
});

state.count = 5; // triggers effect
state.user = "Bob"; // does NOT trigger effect (not accessed in effect)
```

Watch out: Proxy only intercepts the top-level object by default. For deeply
nested state you need to recursively wrap nested objects in their own Proxy —
which is exactly what Vue 3's reactive() does under the hood.
