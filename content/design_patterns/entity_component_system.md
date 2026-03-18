---
title: Entity Component System (ECS)
---

> **Common use cases:** Game engines, simulations, UI frameworks, any system
> with many objects sharing overlapping data

---

## What is ECS?

The **Entity Component System** is a composition-over-inheritance architectural
pattern that separates _identity_ (entities), _data_ (components), and _logic_
(systems). Instead of a class hierarchy where a `Player` extends `Character`,
you build objects by attaching plain data structs to a bare ID.

| Concept       | Role                                      | Rust analogy                                  |
| ------------- | ----------------------------------------- | --------------------------------------------- |
| **Entity**    | A unique ID — nothing more                | `u64` or a newtype wrapper                    |
| **Component** | Plain data attached to an entity          | A `struct` with `#[derive(Component)]`        |
| **System**    | Logic that queries and mutates components | A function that receives queries as arguments |

---

## Simple Example from Scratch

A minimal ECS without any external crate to understand the mechanics:

```rust
use std::collections::HashMap;

type Entity = u64;

// Components are plain structs
#[derive(Debug)]
struct Position { x: f32, y: f32 }

#[derive(Debug)]
struct Velocity { dx: f32, dy: f32 }

// World holds component storage — one HashMap per component type
#[derive(Default)]
struct World {
    next_entity: Entity,
    positions:  HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
}

impl World {
    fn spawn(&mut self) -> Entity {
        let id = self.next_entity;
        self.next_entity += 1;
        id
    }
}

// A system is just a plain function
fn movement_system(world: &mut World) {
    for (entity, vel) in &world.velocities {
        if let Some(pos) = world.positions.get_mut(entity) {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    }
}

fn main() {
    let mut world = World::default();

    let player = world.spawn();
    world.positions.insert(player,  Position  { x: 0.0, y: 0.0 });
    world.velocities.insert(player, Velocity  { dx: 1.0, dy: 0.5 });

    let static_obstacle = world.spawn();
    world.positions.insert(static_obstacle, Position { x: 10.0, y: 10.0 });
    // No Velocity — this entity won't move

    movement_system(&mut world);

    println!("{:?}", world.positions[&player]); // Position { x: 1.0, y: 0.5 }
}
```

Key insight: `static_obstacle` was never touched by `movement_system` because it
has no `Velocity`. Systems only operate on the _intersection_ of components they
care about — no `if entity_is_static` checks required.

---

## Growing the Design

Once the naive HashMap approach gets unwieldy you need:

- **Archetypes** — group entities by their exact component set for better cache
  locality (how Bevy's world works)
- **Sparse sets** — fast add/remove at the cost of iteration speed (used for
  rarely-changed components)
- **System scheduling** — run systems in parallel when they don't share mutable
  access; `bevy_ecs` does this automatically via Rust's borrow rules

```rust
// A slightly more ergonomic API pattern using a builder
struct EntityBuilder<'w> {
    world: &'w mut World,
    id: Entity,
}

impl<'w> EntityBuilder<'w> {
    fn with_position(self, x: f32, y: f32) -> Self {
        self.world.positions.insert(self.id, Position { x, y });
        self
    }
    fn with_velocity(self, dx: f32, dy: f32) -> Self {
        self.world.velocities.insert(self.id, Velocity { dx, dy });
        self
    }
    fn build(self) -> Entity { self.id }
}
```

---

## Bevy as a Production ECS

[Bevy](https://bevyengine.org/) is the most prominent real-world ECS in Rust and
a great study in ergonomic API design. Its ECS lives in the standalone
[`bevy_ecs`](https://crates.io/crates/bevy_ecs) crate — you can use it without
the rest of the game engine.

```rust
use bevy::prelude::*;

// Components — plain structs that derive `Component`
#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { dx: f32, dy: f32 }

// Systems are just functions; Bevy injects queries via its scheduler
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Position { x: 0.0, y: 0.0 },
        Velocity { dx: 1.0, dy: 0.5 },
    ));
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, movement_system)
        .run();
}
```

Notice how `movement_system` never knows about entities at all — it just
expresses _"give me every entity that has both a mutable Position and an
immutable Velocity"_. Bevy's scheduler can then safely parallelise any two
systems whose query sets don't conflict.

### Why Bevy's ECS is a Masterclass in Ergonomic Rust

- **Function-parameter injection** via `SystemParam` traits — adding a
  `Res<Time>` parameter to a system just works
- **`Commands` for deferred mutations** — you can't borrow the world mutably
  while iterating it, so commands queue up spawns/despawns to run between
  systems
- **`With` / `Without` / `Or` filters** — express complex queries in the type
  system with no runtime cost
- **Observers and triggers** (added in Bevy 0.14) — reactive event-driven logic
  built on top of the ECS

---

## Why ECS Fits Rust So Well

Traditional OOP patterns with shared mutable object graphs fight the borrow
checker constantly. ECS sidesteps this by:

1. **Separating data from logic** — systems take fine-grained borrows, so two
   systems can run in parallel as long as they don't both need `&mut` on the
   same component type
2. **Cache-friendly storage** — components of the same type are stored
   contiguously, turning what would be pointer-chasing in OOP into sequential
   memory reads
3. **Composition without `dyn Trait`** — you don't need dynamic dispatch;
   queries are resolved at compile time via associated types and const generics

---

## Resources & Further Reading

### Talks

- 🎥 **Chris Biscardi — "Bevy: A Case Study in Ergonomic Rust"**
  (RustConf 2024)\
  Deep dive into the API design tricks Bevy uses — applicable far beyond games.\
  →
  [https://www.youtube.com/watch?v=CnoDOc6ML0Y](https://www.youtube.com/watch?v=CnoDOc6ML0Y)

- 🎥 **Alice Cecile — "Architecting Bevy"** (interview, 2024)\
  Alice is a core Bevy contributor and foundation member. This talk covers ECS
  architecture decisions and long-term open-source project management.\
  →
  [https://www.youtube.com/watch?v=PND2Wpy6U-E](https://www.youtube.com/watch?v=PND2Wpy6U-E)

### Video Series

- 📺 **Brooks Patton — "Improve Your Rust Skills by Making an ECS Library"**
  (YouTube playlist, 2021)\
  Builds an ECS from scratch in Rust. Covers `TypeId`, `HashMap`, generics,
  `Copy`/`Clone`, interior mutability, and modules — great for understanding the
  internals.\
  →
  [https://www.youtube.com/playlist?list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs](https://www.youtube.com/playlist?list=PLrmY5pVcnuE_SQSzGPWUJrf9Yo-YNeBYs)\
  → Code:
  [https://github.com/brooks-builds/improve_skills_by_building_ecs_library_in_rust](https://github.com/brooks-builds/improve_skills_by_building_ecs_library_in_rust)

### People to Follow

| Person                              | Known for                                                                                                                                                  | Link                                                             |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **Alice Cecile** (`alice-i-cecile`) | Bevy core contributor, ECS architect, RFC author                                                                                                           | [github.com/alice-i-cecile](https://github.com/alice-i-cecile)   |
| **Chris Biscardi**                  | Bevy educator, ergonomic Rust APIs, Rust Adventure                                                                                                         | [youtube/@chrisbiscardi](https://www.youtube.com/@chrisbiscardi) |
| **Brooks Patton** (`brookzerker`)   | ECS from scratch series, Rust std-lib deep dives                                                                                                           | [youtube/@brookzerker](https://www.youtube.com/@brookzerker)     |
| **Jon Gjengset**                    | "Crust of Rust" — intermediate Rust internals (not ECS-specific, but indispensable for understanding the mechanics ECS relies on like interior mutability) | [youtube/@jongjengset](https://www.youtube.com/@jongjengset)     |
| **Alice Ryhl**                      | Tokio maintainer, async Rust expert — not ECS-specific, but her writing on structured concurrency complements ECS scheduling design                        | [ryhl.io](https://ryhl.io)                                       |

### Crates

| Crate                                           | Notes                                                      |
| ----------------------------------------------- | ---------------------------------------------------------- |
| [`bevy_ecs`](https://crates.io/crates/bevy_ecs) | Production-grade, standalone; the reference implementation |
| [`hecs`](https://crates.io/crates/hecs)         | Minimal, low-level; great for embedding                    |
| [`specs`](https://crates.io/crates/specs)       | Older, more explicit; parallel systems via Rayon           |
| [`shipyard`](https://crates.io/crates/shipyard) | Sparse-set ECS; fast add/remove                            |
| [`flecs`](https://crates.io/crates/flecs_ecs)   | Rust bindings for the C flecs library; extremely mature    |

---

## When _Not_ to Use ECS

ECS adds indirection and query overhead. Avoid it when:

- You have **fewer than ~100 objects** that rarely change structure
- Your logic is **purely sequential and single-threaded** with no parallelism
  benefit
- You're building a **CRUD app or domain model** — here, plain structs and trait
  objects are cleaner

A good rule of thumb: reach for ECS when you find yourself writing
`if player.has_component::<Health>()` — that conditional is ECS trying to break
out.
