# bevy_turborand

[![CI](https://github.com/Bluefinger/bevy_turborand/actions/workflows/ci.yml/badge.svg)](https://github.com/Bluefinger/bevy_turborand/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](https://github.com/Bluefinger/bevy_turborand)
[![Cargo](https://img.shields.io/crates/v/bevy_turborand.svg)](https://crates.io/crates/bevy_turborand)
[![Documentation](https://docs.rs/bevy_turborand/badge.svg)](https://docs.rs/bevy_turborand)

A plugin to enable random number generation for the Bevy game engine, built upon [`turborand`](https://github.com/Bluefinger/turborand). Implements ideas from Bevy's [Deterministic RNG RFC](https://github.com/bevyengine/rfcs/pull/55).

`turborand`'s internal implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash), a simple and fast generator but **not** cryptographically secure, as well as [ChaCha8](https://cr.yp.to/chacha.html), a cryptographically secure generator tuned to 8 rounds of the ChaCha algorithm for increased throughput without sacrificing too much security, as per the recommendations in the [Too Much Crypto](https://eprint.iacr.org/2019/1492.pdf) paper.

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
    let mut rng = q_player.single_mut();

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

In order to obtain determinism for your game/app, the `Rng`'s must be seeded. `GlobalRng` and `RngPlugin` can given a seed which then sets the internal PRNG to behave deterministically. Instead of having to seed every `RngComponent` manually, as long as the `GlobalRng` is seeded, then `RngComponent` can be created directly from the global instance, cloning the internal Rng to itself, which gives it a random but deterministic seed. This allows for better randomised states among `RngComponent`s while still having a deterministic app.

Systems also must be ordered correctly for determinism to occur. Systems however do not need to be strictly ordered against every one as if some linear path. Only related systems that access a given set of `RngComponent`s need to be ordered. Ones that are unrelated can run in parallel and still yield a deterministic result. So systems selecting a `Player` entity with a `RngComponent` should all be ordered against each other, but systems selecting an `Item` entity with an `RngComponent` that never interacts with `Player` don't need to be ordered with `Player` systems, only between themselves.

To see an example of this, view the [project's tests](tests/determinism.rs) to see how to make use of determinism for testing random systems.

## Migration Guide from 0.2 to 0.3

With `turborand` 0.6, there are a lot of breaking changes due to a rework of the API. For the most part, this is mostly internal to `turborand` and `bevy_turborand` exposes the new traits by default, so any existing code should more or less work fine, except for the following:

- `from_global` on `RngComponent` no longer exists. Instead, there are `From` implementations on `RngComponent` and `ChaChaRngComponent` that cover more use-cases where a reference or resource or even another component could be used for initialising a new `RngComponent` and so forth. This makes it more flexible with regards to what the source is. Just go to town with `RngComponent::from`.
- `new` functions no longer accept an Option parameter for the seed. The methods are split between `new` for initialising without a seed (so obtaining a random seed), and `with_seed` for initialising with a seed value (which applies to components and resources).
- `RngPlugin` now uses a builder pattern to initialise. `new` creates a default state with no seeds applied, and then `with_rng_seed` and `with_chacha_seed` applies seed values to the plugin to then initialise the global RNG resources with. See the docs for an example of how that might look now.
- `bevy_turborand` now has a number of feature flags, and apart from the new traits (which are alwways provided when no flags are enabled), everything else is behind a feature flag. For example, `wyrand` based structs (`RngComponent` et al) are behind the `wyrand` flag, which is enabled by default. For a higher quality entropy source (though it will be slower), `chacha` flag provides RNG provided by the ChaCha8 algorithm, such as `ChaChaRngComponent`. `RngPlugin` is available when either `wyrand` or `chacha` is enabled. Otherwise, existing flags like `rand` enable the rand crate compatibility layer, and `serialize` for serde derives.

## Migration Guide from 0.3 to 0.4

`bevy_turborand` moves to `turborand` 0.8, which rolls with a couple of major API breaking changes. Certain traits are no longer exposed as they are internal implementation details. The main changes are that `ChaChaRng` is no longer backed by a `Vec` for buffering entropy, switching to an aligned array for improving generation throughput at the slight cost of initialisation performance and struct size. It does mean no need for the single heap allocation however when the RNG generates a number for the first time. This refactor also changes how `ChaChaRng` is serialised, so `bevy_turborand` 0.4 is not compatible with previously serialised data.

Also, the old `Clone` behaviour for `TurboCore` RNGs has been changed, so `.clone()` now maintains the state between original and cloned instances. The old behaviour now exists as the `ForkableCore` trait with the `.fork()` method, which has the original instance's state be mutated in order to derive a new random state for the forked instance. As such, `RngComponent` and `ChaChaRngComponent` can now implement `Clone`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
