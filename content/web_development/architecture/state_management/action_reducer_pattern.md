---
title: Action/Reducer Pattern
---

## Redux-Like Action/Reducer Pattern

```
┌──────────────────────────┐
│        UI / View         │
│  (components, widgets)   │
└────────────┬─────────────┘
             │
             │ dispatch(action)
             ▼
      ┌───────────────┐
      │    ACTION     │
      │ type + data   │
      └──────┬────────┘
             │
             │ sent to
             ▼
┌──────────────────────────────────────┐
│   STORE (currentState+ reducer)      │
└────────────┬─────────────────────────┘
             │
             │ calls
             ▼
   ┌───────────────────────┐
   │       REDUCER         │
   │  newState = f(        │
   │    state, action      │
   │  )                    │
   └─────────┬─────────────┘
             │
             │ returns newState
             ▼
      ┌───────────────┐
      │   STORE       │
      │ updates state │
      └──────┬────────┘
             │
             │ notifies subscribers
             ▼
┌───────────────────────────┐
│        UI / View          │
│ re-renders from state     │
└───────────────────────────┘
```

For more predictable updates, you can implement a Redux-style pattern using
typed discriminated unions:

```ts
type Action =
  | { type: "ADD_TODO"; payload: { text: string } }
  | { type: "TOGGLE_TODO"; payload: { id: number } };

function reducer(state: AppState, action: Action): AppState {
  switch (action.type) {
    case "ADD_TODO":
      return { ...state, todos: [...state.todos, action.payload] };
    case "TOGGLE_TODO":
      return { ...state /* ... */ };
    default:
      return state;
  }
}
```

TypeScript's exhaustive checking ensures your switch covers every action type,
preventing silent missed cases.
