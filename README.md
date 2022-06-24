# bevy_turborand

[![CI](https://github.com/Bluefinger/bevy_turborand/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/bevy_turborand/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/bevy_turborand)
[![Cargo](https://img.shields.io/crates/v/bevy_turborand.svg)](https://crates.io/crates/bevy_turborand)
[![Documentation](https://docs.rs/bevy_turborand/badge.svg)](https://docs.rs/bevy_turborand)

A plugin to enable random number generation for the Bevy game engine, built upon [`turborand`](https://github.com/Bluefinger/turborand).
Implements ideas from Bevy's [Deterministic RNG RFC](https://github.com/bevyengine/rfcs/pull/55).

`turborand`'s internal implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast
generator but **not** cryptographically secure.

## Example

```rust
use bevy::prelude::*;
use bevy_turborand::*;

#[derive(Debug, Component)]
struct Player;

fn setup_player(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.spawn()
        .insert(Player)
        .insert(RngComponent::from_global(&mut global_rng));
}

fn do_damage(mut q_player: Query<&mut RngComponent, With<Player>>) {
    let rng = q_player.single_mut();
    println!("Player attacked for {} damage!", rng.u32(10..=20));
}

fn main() {
    App::new()
        .add_plugin(RngPlugin::default())
        .add_startup_system(setup_player)
        .add_system(do_damage)
        .run();
}
```

## Deterministic RNG

In order to obtain determinism for your game/app, the `Rng`'s must be
seeded. `GlobalRng` and `RngPlugin` can given a seed which then sets the
internal PRNG to behave deterministically. Instead of having to seed every
`RngComponent` manually, as long as the `GlobalRng` is seeded, then
`RngComponent` can be created directly from the global instance, cloning
the internal Rng to itself, which gives it a random but deterministic seed.
This allows for better randomised states among `RngComponent`s while still
having a deterministic app.

Systems also must be ordered correctly for determinism to occur. Systems
however do not need to be strictly ordered against every one as if some
linear path. Only related systems that access a given set of `RngComponent`s
need to be ordered. Ones that are unrelated can run in parallel and still
yield a deterministic result. So systems selecting a `Player` entity with 
a `RngComponent` should all be ordered against each other, but systems
selecting an `Item` entity with an `RngComponent` that never interacts with
`Player` don't need to be ordered with `Player` systems, only between
themselves.

To see an example of this, view the [project's tests](tests/determinism.rs) to
see how to make use of determinism for testing random systems.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
