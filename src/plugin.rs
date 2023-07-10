use crate::*;

/// A [`Plugin`] for initialising a [`GlobalRng`] & [`GlobalChaChaRng`]
/// (if the feature flags are enabled for either of them) into a Bevy `App`.
/// Also registers the types for reflection support if `serialize` feature flag
/// is enabled.
///
/// # Example
/// ```
/// use bevy::prelude::*;
/// use bevy_turborand::prelude::*;
///
/// App::new()
///     .add_plugins(RngPlugin::new().with_rng_seed(12345))
///     .run();
///
/// ```
#[cfg_attr(docsrs, doc(cfg(any(feature = "wyrand", feature = "chacha"))))]
pub struct RngPlugin {
    #[cfg(feature = "wyrand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wyrand")))]
    rng: Option<u64>,
    #[cfg(feature = "chacha")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chacha")))]
    chacha: Option<[u8; 40]>,
}

impl RngPlugin {
    /// Create a new [`RngPlugin`] instance with no seeds provided by default.
    /// If initialised as is, this will set the RNGs to have randomised seeds.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "wyrand")]
            rng: None,
            #[cfg(feature = "chacha")]
            chacha: None,
        }
    }

    /// Builder function to set a seed value for a [`GlobalRng`].
    #[cfg(feature = "wyrand")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wyrand")))]
    #[inline]
    #[must_use]
    pub const fn with_rng_seed(mut self, seed: u64) -> Self {
        self.rng = Some(seed);
        self
    }

    /// Builder function to set a seed value for a [`GlobalChaChaRng`].
    #[cfg(feature = "chacha")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chacha")))]
    #[inline]
    #[must_use]
    pub const fn with_chacha_seed(mut self, seed: [u8; 40]) -> Self {
        self.chacha = Some(seed);
        self
    }
}

impl Default for RngPlugin {
    /// Creates a default [`RngPlugin`] instance. The RNG instances will
    /// be initialised with randomised seeds, so this is **not**
    /// deterministic.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(all(feature = "wyrand", feature = "serialize"))]
        app.register_type::<RngComponent>()
            .register_type::<GlobalRng>();

        #[cfg(feature = "wyrand")]
        app.insert_resource(self.rng.map_or_else(GlobalRng::new, GlobalRng::with_seed));

        #[cfg(all(feature = "chacha", feature = "serialize"))]
        app.register_type::<ChaChaRngComponent>()
            .register_type::<GlobalChaChaRng>();

        #[cfg(feature = "chacha")]
        app.insert_resource(
            self.chacha
                .map_or_else(GlobalChaChaRng::new, GlobalChaChaRng::with_seed),
        );
    }
}
