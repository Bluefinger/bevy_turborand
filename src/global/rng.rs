use crate::*;

/// A Global [`Rng`] instance, meant for use as a Resource. Gets
/// created automatically with [`RngPlugin`], or can be created
/// and added manually.
#[derive(Debug, Clone, Resource, PartialEq, Reflect)]
#[cfg_attr(docsrs, doc(cfg(feature = "wyrand")))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    reflect_value(Debug, PartialEq, Default, Serialize, Deserialize)
)]
#[cfg_attr(not(feature = "serialize"), reflect_value(Debug, PartialEq, Default))]
pub struct GlobalRng(#[reflect(default)] Rng);

unsafe impl Sync for GlobalRng {}

impl GlobalRng {
    /// Create a new [`GlobalRng`] instance with a randomised seed.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(Rng::new())
    }

    /// Create a new [`GlobalRng`] instance with a given seed.
    #[inline]
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self(Rng::with_seed(seed))
    }
}

impl DelegatedRng for GlobalRng {
    type Source = Rng;

    /// Returns the internal [`TurboRand`] reference. Useful
    /// for working directly with the internal [`TurboRand`], such as
    /// needing to pass the [`TurboRand`] into iterators.
    ///
    /// # Example
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_turborand::prelude::*;
    /// use std::iter::repeat_with;
    ///
    /// fn contrived_random_actions(mut rand: ResMut<GlobalRng>) {
    ///     let rand = rand.get_mut(); // Important to shadow the rand mut reference into being an immutable `TurboRand` one.
    ///
    ///     // Now the `TurboRand` instance can be borrowed in multiple places in the iterator without issue.
    ///     let output: Vec<f64> = repeat_with(|| rand.f64()).take(5).filter(|&val| rand.chance(val)).collect();
    ///
    ///     println!("Received random values: {:?}", output);
    /// }
    /// ```
    #[inline]
    fn get_mut(&mut self) -> &mut Self::Source {
        &mut self.0
    }
}

impl Default for GlobalRng {
    /// Creates a default [`GlobalRng`] instance. The instance will
    /// be initialised with a randomised seed, so this is **not**
    /// deterministic.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl AsMut<Rng> for GlobalRng {
    fn as_mut(&mut self) -> &mut Rng {
        self.get_mut()
    }
}
