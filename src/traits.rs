use crate::{SeededCore, TurboCore, TurboRand};
use std::{fmt::Debug, ops::RangeBounds};

#[cfg(feature = "rand")]
use crate::RandBorrowed;

/// Something
pub trait DelegatedRng
where
    Self::Source: Default + Debug + Clone + PartialEq + TurboCore + TurboRand + SeededCore,
{
    /// The source
    type Source;

    /// Returns the internal [`TurboRand`] reference. Useful
    /// for working directly with the internal [`TurboRand`], such as
    /// needing to pass the [`TurboRand`] into iterators.
    ///
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_turborand::*;
    /// use std::iter::repeat_with;
    ///
    /// fn contrived_random_actions(mut q_rand: Query<&mut RngComponent>) {
    ///     for mut rand in q_rand.iter_mut() {
    ///         let rand = rand.get_mut(); // Important to shadow the rand mut reference into being an immutable `Rng` one.
    ///
    ///         // Now the `TurboRand` source can be borrowed in multiple places in the iterator without issue.
    ///         let output: Vec<f64> = repeat_with(|| rand.f64()).take(5).filter(|&val| rand.chance(val)).collect();
    ///
    ///         println!("Received random values: {:?}", output);
    ///     }
    /// }
    /// ```
    fn get_mut(&mut self) -> &mut Self::Source;

    /// Reseeds the [`DelegatedRng`] with a new seed/state, resolving to the
    /// seed type of the underlying [`SeededCore`] instance.
    #[inline]
    fn reseed(&mut self, seed: <<Self as DelegatedRng>::Source as SeededCore>::Seed) {
        self.get_mut().reseed(seed);
    }

    /// Return a compatibility shim for working with crates from the `rand`
    /// ecosystem.
    #[cfg(feature = "rand")]
    #[inline]
    fn as_rand(&mut self) -> RandBorrowed<'_, Self::Source> {
        RandBorrowed::from(self.get_mut())
    }

    delegate_rng_trait!(
        u128,
        u128,
        impl RangeBounds<u128>,
        "Delegated [`TurboRand::u128`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        u64,
        u64,
        impl RangeBounds<u64>,
        "Delegated [`TurboRand::u64`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        u32,
        u32,
        impl RangeBounds<u32>,
        "Delegated [`TurboRand::u32`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        u16,
        u16,
        impl RangeBounds<u16>,
        "Delegated [`TurboRand::u16`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        u8,
        u8,
        impl RangeBounds<u8>,
        "Delegated [`TurboRand::u8`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        i128,
        i128,
        impl RangeBounds<i128>,
        "Delegated [`TurboRand::i128`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        i64,
        i64,
        impl RangeBounds<i64>,
        "Delegated [`TurboRand::i64`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        i32,
        i32,
        impl RangeBounds<i32>,
        "Delegated [`TurboRand::i32`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        i16,
        i16,
        impl RangeBounds<i16>,
        "Delegated [`TurboRand::i16`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        i8,
        i8,
        impl RangeBounds<i8>,
        "Delegated [`TurboRand::i8`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        usize,
        usize,
        impl RangeBounds<usize>,
        "Delegated [`TurboRand::usize`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        isize,
        isize,
        impl RangeBounds<isize>,
        "Delegated [`TurboRand::isize`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        chance,
        bool,
        f64,
        "Delegated [`TurboRand::chance`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        char,
        char,
        impl RangeBounds<char>,
        "Delegated [`TurboRand::char`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        digit,
        char,
        u8,
        "Delegated [`TurboRand::digit`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        alphabetic,
        char,
        "Delegated [`TurboRand::alphabetic`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        alphanumeric,
        char,
        "Delegated [`TurboRand::alphanumeric`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        lowercase,
        char,
        "Delegated [`TurboRand::lowercase`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        uppercase,
        char,
        "Delegated [`TurboRand::uppercase`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        bool,
        bool,
        "Delegated [`TurboRand::bool`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        f64,
        f64,
        "Delegated [`TurboRand::f64`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        f32,
        f32,
        "Delegated [`TurboRand::f32`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        f64_normalized,
        f64,
        "Delegated [`TurboRand::f64_normalized`] method from [`TurboRand`]."
    );
    delegate_rng_trait!(
        f32_normalized,
        f32,
        "Delegated [`TurboRand::f32_normalized`] method from [`TurboRand`]."
    );

    /// Delegated [`TurboCore::fill_bytes`] method from [`TurboCore`].
    #[inline]
    fn fill_bytes<B: AsMut<[u8]>>(&mut self, buffer: B) {
        self.get_mut().fill_bytes(buffer);
    }

    /// Delegated [`TurboRand::shuffle`] method from [`TurboRand`].
    #[inline]
    fn shuffle<T>(&mut self, slice: &mut [T]) {
        self.get_mut().shuffle(slice);
    }

    /// Delegated [`TurboRand::sample`] method from [`TurboRand`].
    #[inline]
    fn sample<'a, T>(&mut self, list: &'a [T]) -> Option<&'a T> {
        self.get_mut().sample(list)
    }

    /// Delegated [`TurboRand::sample_multiple`] method from [`TurboRand`].
    #[inline]
    fn sample_multiple<'a, T>(&mut self, list: &'a [T], amount: usize) -> Vec<&'a T> {
        self.get_mut().sample_multiple(list, amount)
    }

    /// Delegated [`TurboRand::weighted_sample`] method from [`TurboRand`].
    #[inline]
    fn weighted_sample<'a, T, F>(&mut self, list: &'a [T], weight_sampler: F) -> Option<&'a T>
    where
        F: Fn(&'a T) -> f64,
    {
        self.get_mut().weighted_sample(list, weight_sampler)
    }
}
