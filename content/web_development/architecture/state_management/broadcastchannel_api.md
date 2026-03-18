---
title: Broadcast Channel API
---

```
BroadcastChannel (cross-context bus)
---------------------------------------

     [ Tab 1 ]           [ Tab 2 ]             [ Worker / Tab 3 ]
   +-----------+       +-----------+           +-----------------+
   | App code  |       | App code  |           |  App code       |
   | publishes |       | subscribes|           |  subscribes     |
   +-----+-----+       +-----+-----+           +--------+--------+
         |                   |                         |
         +---------+---------+-------------------------+
                   v
          +-----------------------+
          |  BroadcastChannel     |   browser-level bus
          |  ("app-events")       |
          +-----------------------+
```

`BroadcastChannel` is a built-in browser API that lets different browser
contexts (tabs, iframes, workers) on the same origin communicate by posting
messages to a named channel. It's essentially a pub/sub bus at the browser level
— perfect for syncing state across tabs without a server.

## Typed BroadcastChannel in TypeScript

You can wrap it in a typed class to get full compile-time safety:

```ts
type StateMessage<T> =
  | { type: "STATE_UPDATE"; payload: Partial<T> }
  | { type: "REQUEST_SYNC" };

class SyncedStore<T extends object> {
  private state: T;
  private channel: BroadcastChannel;
  private subscribers = new Set<(s: T) => void>();

  constructor(name: string, initial: T) {
    this.state = { ...initial };
    this.channel = new BroadcastChannel(name);
    this.channel.onmessage = (e: MessageEvent<StateMessage<T>>) => {
      if (e.data.type === "STATE_UPDATE") {
        this.state = { ...this.state, ...e.data.payload };
        this.subscribers.forEach((cb) => cb(this.state));
      }
    };
  }

  setState(updates: Partial<T>) {
    this.state = { ...this.state, ...updates };
    this.channel.postMessage({ type: "STATE_UPDATE", payload: updates });
    this.subscribers.forEach((cb) => cb(this.state));
  }

  subscribe(cb: (s: T) => void) {
    this.subscribers.add(cb);
    return () => this.subscribers.delete(cb);
  }

  destroy() {
    this.channel.close();
  }
}
```

TypeScript's MessageEvent<T> generic ensures the e.data payload is typed
correctly, preventing you from accidentally posting or reading the wrong shape. ​

### Common Use Cases

- Session sync — if a user logs out in one tab, broadcast a logout action to
  immediately reflect that across all open tabs ​

- Shopping cart sync — changes made in one tab propagate instantly to others

- Shared worker state — coordinate state between a ServiceWorker and page tabs

### Limitations to Know

- BroadcastChannel does not fire in the tab that sent the message — only other
  tabs receive it, so you still update local state directly ​

- It only works within the same origin (same protocol + domain + port)

- It has no built-in message history — a newly opened tab won't get past state
  unless you implement a REQUEST_SYNC handshake pattern (one tab requests the
  current state, another responds with it)

- Not available in IE, but has full support in all modern browsers

This pattern pairs naturally with the observable store from the previous answer
— the BroadcastChannel becomes an external subscriber that mirrors state updates
to other tabs.
