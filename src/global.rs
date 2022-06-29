use crate::*;

/// A Global [`Rng`] instance, meant for use as a Resource. Gets
/// created automatically with [`RngPlugin`], or can be created
/// and added manually.
#[derive(Debug, Deref)]
pub struct GlobalRng(Rng<AtomicState>);

impl GlobalRng {
    /// Create a new [`GlobalRng`] instance with an optional seed value.
    /// Uses a randomised seed if `None` is provided.
    #[inline]
    #[must_use]
    pub fn new(seed: Option<u64>) -> Self {
        match seed {
            Some(s) => Self(atomic_rng!(s)),
            None => Self(atomic_rng!()),
        }
    }
}

impl Default for GlobalRng {
    /// Creates a default [`GlobalRng`] instance. The instance will
    /// be initialised with a randomised seed, so this is **not**
    /// deterministic.
    fn default() -> Self {
        Self::new(None)
    }
}