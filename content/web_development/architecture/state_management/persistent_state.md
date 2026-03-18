---
title: Persistent State
---

```
               +-------------------+
setState() --> |  PersistentStore  | --> subscribers notified
               +-------------------+
                      |    ^
                  save|    |load on init
                      v    |
               +-------------------+
               |   localStorage    |
               +-------------------+
```

State that survives page reloads by syncing to localStorage (or sessionStorage).
Includes a _version field so you can safely migrate stale data from older
schemas.

```ts
interface Versioned {
  _version: number;
}

class PersistentStore<T extends Versioned> {
  private state: T;
  private subscribers = new Set<(s: T) => void>();
  private key: string;
  private currentVersion: number;
  private migrate: (old: any) => T;

  constructor(key: string, initial: T, migrate: (old: any) => T) {
    this.key = key;
    this.currentVersion = initial._version;
    this.migrate = migrate;
    this.state = this.load(initial);
  }

  private load(initial: T): T {
    const raw = localStorage.getItem(this.key);
    if (!raw) return initial;
    const parsed = JSON.parse(raw);
    if (parsed._version !== this.currentVersion) return this.migrate(parsed);
    return parsed as T;
  }

  private save() {
    localStorage.setItem(this.key, JSON.stringify(this.state));
  }

  getState(): T {
    return { ...this.state };
  }

  setState(updates: Partial<T>) {
    this.state = { ...this.state, ...updates };
    this.save();
    this.subscribers.forEach((cb) => cb(this.getState()));
  }

  subscribe(cb: (s: T) => void) {
    this.subscribers.add(cb);
    return () => this.subscribers.delete(cb);
  }
}

// Usage
interface ThemeState extends Versioned {
  theme: "light" | "dark";
  fontSize: number;
}

const themeStore = new PersistentStore<ThemeState>(
  "theme",
  { _version: 2, theme: "light", fontSize: 16 },
  (old) => ({ _version: 2, theme: old.theme ?? "light", fontSize: 16 }), // migrate v1 → v2
);

themeStore.setState({ theme: "dark" }); // persisted to localStorage immediately
```

Use case: User preferences, session data, or any state that should survive a
page refresh — theme, language, last-visited route, partially filled forms.
