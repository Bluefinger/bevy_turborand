//! A plugin to enable ECS optimised random number generation for the Bevy game engine.
//!
//! The plugin makes use of [`turborand`](https://docs.rs/turborand/latest/turborand/),
//! on which the implementation uses [Wyrand](https://github.com/wangyi-fudan/wyhash),
//! a simple and fast generator but **not** cryptographically secure, as well as
//! [ChaCha8](https://cr.yp.to/chacha.html), a cryptographically secure generator tuned
//! to 8 rounds of the ChaCha algorithm, for the purpose of increasing throughput at the
//! expense of slightly less security (though plenty secure enough for cryptographic purposes).
//!
//! This plugin exposes [`GlobalRng`] & [`GlobalChaChaRng`] for use as a `Resource`,
//! as well as [`RngComponent`] & [`ChaChaRngComponent`] for providing rng instances
//! at a per-entity level. By exposing random number generation as a component allows
//! for better parallelisation of systems making use of PRNG, as well as making it
//! easier to enable determinism in an otherwise multi-threaded engine.
//! Relying on a single `Rng` instance for the entire application is not
//! conducive to multi-threading, and imposes far too strict ordering
//! requirements in order to ensure that each time the Rng is called and its
//! internal state is modified, that it is done so in a deterministic manner.
//!
//! By splitting one instance into components, each [`RngComponent`] only is
//! responsible for the entity it is applied to. It also prevents other
//! actions in the game from affecting the outcome of unrelated entities.
//! Also, Bevy's queries are not stable in their ordering, so each time
//! a system runs, the order by which a query iterates through selected
//! entities will be different. By providing an instance of an [`Rng`] per
//! entity, it then makes the question of stable ordering in queries moot.
//! Thus, determinism can be achieved regardless of unstable query ordering
//! and multi-threaded execution.
//! 
//! ## Notice
//!
//! For all intents and purposes, `bevy_turborand` will no longer receive new features
//! or work, and is mostly on maintenance only mode. I will keep this crate up-to-date
//! with bevy releases, but otherwise all new work and efforts is currently directed towards
//! [`bevy_rand`](https://github.com/Bluefinger/bevy_rand). Folks who wish to add more
//! capability to this crate are free to submit PRs.
//!
//! # Usage
//!
//! For both global and component RNGs, both must be accessed with a
//! `mut` reference in order to ensure each system using the [`TurboCore`] instances
//! are not run in parallel over the entities being accessed. This should
//! also allow for better diagnostics with Bevy's ambiguous ordering tool
//! for finding systems that should be more explicitly ordered.
//!
//! On its own, the [`TurboCore`] is not threadsafe unless it is accessed via a `mut`
//! reference. By doing so, the RNG can be even more performant and not
//! have to rely on atomics (which impose considerable overhead).
//!
//! After requesting [`GlobalRng`] resource or the [`RngComponent`], one can use
//! delegated methods directly to get random numbers that way, or call `.get_mut()`
//! in order to get the [`TurboCore`] instance itself. From there, all [`TurboRand`] methods in
//! [`turborand`](https://docs.rs/turborand/latest/turborand/) are available to be
//! used, though most are available as delegated methods in [`GlobalRng`] and
//! [`RngComponent`]. The same applies to [`GlobalChaChaRng`] and
//! [`ChaChaRngComponent`].
//!
//! [`GlobalRng`] is provided as a means to seed [`RngComponent`] with randomised
//! states, and should **not** be used as a direct source of entropy for
//! systems in general. All systems that access the [`GlobalRng`] cannot be
//! parallelised easily, so [`RngComponent`] should be used instead as much as
//! possible. On the plus side with [`GlobalRng`], only one seed needs to be
//! provided in order to have all instances be deterministic, as long as all
//! [`RngComponent`]s are created using [`GlobalRng`]. [`RngComponent`] can also
//! be used to seed other [`RngComponent`]s.
//!
//! **Note**: [`GlobalChaChaRng`] & [`ChaChaRngComponent`] can seed both [`ChaChaRngComponent`]
//! and [`RngComponent`]. However, [`GlobalRng`] & [`RngComponent`] cannot be used
//! to seed [`ChaChaRngComponent`] as they don't implement [`SecureCore`].
//! You can only seed from high quality to same quality entropy sources, but never from
//! worse quality entropy sources.
//!
//! # Example
//!
//! Basic example of setting up and using the Rng.
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_turborand::prelude::*;
//!
//! #[derive(Debug, Component)]
//! struct Player;
//!
//! fn setup_player(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
//!     commands.spawn((
//!             Player,
//!             RngComponent::from(&mut global_rng),
//!         ));
//! }
//!
//! fn do_damage(mut q_player: Query<&mut RngComponent, With<Player>>) {
//!     let mut rng = q_player.single_mut().unwrap();
//!
//!     println!("Player attacked for {} damage!", rng.u32(10..=20));
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(RngPlugin::default())
//!         .add_systems(Startup, setup_player)
//!         .add_systems(Update, do_damage)
//!         .run();
//! }
//! ```
//!
//! # How to enable Determinism
//!
//! In order to obtain determinism for your game/app, the [`TurboRand`] sources must be
//! seeded. [`GlobalRng`] and [`RngPlugin`] can given a seed which then sets the
//! internal PRNG to behave deterministically. Instead of having to seed every
//! [`RngComponent`] manually, as long as the [`GlobalRng`] is seeded, then
//! [`RngComponent`] can be created directly from the global instance, cloning
//! the internal Rng to itself, which gives it a random but deterministic seed.
//! This allows for better randomised states among [`RngComponent`]s while still
//! having a deterministic app. [`RngComponent`]s derived from a [`GlobalRng`]
//! can also then seed other [`RngComponent`]s in a deterministic manner.
//!
//! Systems also must be ordered correctly for determinism to occur. Systems
//! however do not need to be strictly ordered against every one as if some
//! linear path. Only related systems that access a given set of [`RngComponent`]s
//! need to be ordered. Ones that are unrelated can run in parallel and still
//! yield a deterministic result. So systems selecting a `Player` entity with
//! a [`RngComponent`] should all be ordered against each other, but systems
//! selecting an `Item` entity with an [`RngComponent`] that never interacts with
//! `Player` don't need to be ordered with `Player` systems, only between
//! themselves.
//!
//! To see an example of this, view the project's tests to see how to make
//! use of determinism for testing random systems.
//!
//! # Caveats about Determinism
//!
//! Usage of the [`TurboRand`] method [`TurboRand::usize`] will not exhibit the same result
//! on 64-bit systems and 32-bit systems. The method output will be different
//! on those platforms, though it will be deterministically different. This is
//! because the output of the RNG source for usize on 32-bit platforms
//! is `u32` and thus is truncating the full output from the generator.
//! As such, it will not be the same value between 32-bit and 64-bit platforms.
//!
//! For ensuring stable results between 32-bit and 64-bit platforms, use the [`TurboRand::index`]
//! method instead. All sampling/shuffing methods use this method internally to ensure
//! stable results. Do note, [`TurboRand`] optimises cases for 64-bit platforms,
//! as these are much more common for general and game applications.
//!
//! # Features
//!
//! - **`wyrand`** - Enables [`GlobalRng`] & [`RngComponent`]. Is enabled by default.
//!   Having this feature flag enabled also enables [`RngPlugin`].
//! - **`chacha`** - Enables [`GlobalChaChaRng`] & [`ChaChaRngComponent`]. Having this
//!   feature flag enabled also enables [`RngPlugin`].
//! - **`rand`** - Provides [`RandBorrowed`], which implements `RngCore`
//!   so to allow for compatibility with `rand` ecosystem of crates.
//! - **`serialize`** - Enables [`Serialize`] and [`Deserialize`] derives.
#![warn(missing_docs, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

use bevy::prelude::*;
#[cfg(feature = "chacha")]
use turborand::prelude::ChaChaRng;
#[cfg(feature = "wyrand")]
use turborand::prelude::Rng;
pub use turborand::{ForkableCore, GenCore, SecureCore, SeededCore, TurboCore, TurboRand};

#[cfg(all(any(feature = "chacha", feature = "wyrand"), feature = "serialize"))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "chacha")]
pub use component::chacha::*;
#[cfg(feature = "wyrand")]
pub use component::rng::*;
#[cfg(feature = "chacha")]
pub use global::chacha::*;
#[cfg(feature = "wyrand")]
pub use global::rng::*;
pub use traits::*;

#[macro_use]
mod delegate;
#[cfg(any(feature = "chacha", feature = "wyrand"))]
mod component;
#[cfg(any(feature = "chacha", feature = "wyrand"))]
mod global;
#[cfg(any(feature = "wyrand", feature = "chacha"))]
mod plugin;
mod traits;

/// Prelude for `bevy_turborand`, exposing all necessary traits for default usage of the
/// crate, as well as whatever component/resources are configured to be exposed by whichever
/// features are enabled.
pub mod prelude;

/// Module for dealing directly with [`turborand`] and its features.
///
/// # Examples
///
/// Generate a random value:
///
/// ```
/// use bevy_turborand::rng::*;
///
/// let rand = Rng::new();
///
/// let value = rand.bool();
/// ```
///
/// Sample a value from a list:
///
/// ```
/// use bevy_turborand::rng::*;
///
/// let rand = Rng::new();
///
/// let values = [1, 2, 3, 4, 5];
///
/// let value = rand.sample(&values);
/// ```
///
/// Generate a vector with random values:
///
/// ```
/// use bevy_turborand::rng::*;
/// use std::iter::repeat_with;
///
/// let rand = Rng::new();
///
/// let values: Vec<_> = repeat_with(|| rand.f32()).take(10).collect();
/// ```
pub mod rng {
    pub use turborand::prelude::*;
}
