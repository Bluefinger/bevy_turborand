use crate::*;

/// A Global [`SecureRng`] instance, meant for use as a Resource. Gets
/// created automatically with [`RngPlugin`], or can be created
/// and added manually.
#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct GlobalSecureRng(SecureRng);

unsafe impl Sync for GlobalSecureRng {}

impl GlobalSecureRng {
    /// Create a new [`GlobalSecureRng`] instance with a randomised seed.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(SecureRng::new())
    }

    /// Create a new [`GlobalSecureRng`] instance with a given seed.
    #[inline]
    #[must_use]
    pub fn with_seed(seed: [u8; 40]) -> Self {
        Self(SecureRng::with_seed(seed))
    }
}

impl DelegatedRng for GlobalSecureRng {
    type Source = SecureRng;

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
    /// fn contrived_random_actions(mut rand: ResMut<GlobalSecureRng>) {
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

impl Default for GlobalSecureRng {
    /// Creates a default [`GlobalSecureRng`] instance. The instance will
    /// be initialised with a randomised seed, so this is **not**
    /// deterministic.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl AsMut<SecureRng> for GlobalSecureRng {
    fn as_mut(&mut self) -> &mut SecureRng {
        self.get_mut()
    }
}
