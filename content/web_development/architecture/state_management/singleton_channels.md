---
title: Singleton Channel
---

```
Singleton Channel (neutral in-app bus)
-----------------------------------------

   Publishers                          Subscribers
   ----------                          -----------
   +--------+                          +--------+
   | View A |----+                 +-->| View C |
   +--------+    |                 |   +--------+
                 v                 |
            +---------------------------+
            |     AppChannel<Event>     |  (singleton)
            +---------------------------+
                 ^                 |
   +--------+    |                 |   +--------+
   | Store  |----+                 +-->| Logger |
   +--------+                          +--------+
```

A `Channel<T>` is a globally shared, typed event bus. Any component can publish
a value of type T into it, and any component that has subscribed will receive
that value — FIFO (first subscriber registered, first notified). It's simpler
than a full store because it carries no persistent state; it just broadcasts a
moment-in-time event.

```ts
class Channel<T> {
  private subscribers: Array<(data: T) => void> = [];

  subscribe(cb: (data: T) => void): () => void {
    this.subscribers.push(cb);
    return () => {
      this.subscribers = this.subscribers.filter((s) => s !== cb);
    };
  }

  publish(data: T): void {
    for (const cb of this.subscribers) {
      cb(data);
    }
  }
}
```

Because it's generic, TypeScript enforces that every publisher and subscriber
agrees on the shape of `T` at compile time — you can't accidentally publish a
`string` on a `Channel<UserEvent>`.

## Defining Global Singleton Channels

You declare each channel once and export it as a module-level singleton:

```ts
// channels.ts
interface UserLoggedIn {
  userId: string;
  role: "admin" | "user";
}
interface CartUpdated {
  itemCount: number;
  total: number;
}

export const userLoginChannel = new Channel<UserLoggedIn>();
export const cartChannel = new Channel<CartUpdated>();
```

Any file can import and use these without passing them through props or
constructors.

## Usage Across Components

```ts
// ComponentA.ts — subscriber
import { cartChannel } from './channels';

const unsub = cartChannel.subscribe(({ itemCount, total }) => {
  document.getElementById('cart-count')!.textContent = String(itemCount);
});

// call unsub() when component is destroyed to avoid memory leaks

// ComponentB.ts — publisher
import { cartChannel } from './channels';

function addToCart(item: Item) {
  // ... update cart logic
  cartChannel.publish({ itemCount: cart.length, total: cart.reduce(...) });
}
```
