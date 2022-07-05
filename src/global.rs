use crate::*;

/// A Global [`Rng`] instance, meant for use as a Resource. Gets
/// created automatically with [`RngPlugin`], or can be created
/// and added manually.
#[derive(Debug)]
pub struct GlobalRng(Rng<CellState>);

unsafe impl Sync for GlobalRng {}

impl GlobalRng {
    /// Create a new [`GlobalRng`] instance with an optional seed value.
    /// Uses a randomised seed if `None` is provided.
    #[inline]
    #[must_use]
    pub fn new(seed: Option<u64>) -> Self {
        match seed {
            Some(s) => Self(rng!(s)),
            None => Self(rng!()),
        }
    }

    /// Returns the internal [`Rng<CellState>`] reference. Useful
    /// for working directly with the internal [`Rng`], such as
    /// needing to pass the [`Rng`] into iterators.
    #[inline]
    pub fn get_mut(&mut self) -> &mut Rng<CellState> {
        &mut self.0
    }

    /// Reseeds the [`GlobalRng`] with a new seed/state.
    #[inline]
    pub fn reseed(&mut self, seed: u64) {
        self.get_mut().reseed(seed);
    }

    delegate_rng!(
        u128,
        u128,
        impl RangeBounds<u128>,
        "Delegated [`Rng::u128`] method from [`Rng`]."
    );
    delegate_rng!(
        u64,
        u64,
        impl RangeBounds<u64>,
        "Delegated [`Rng::u64`] method from [`Rng`]."
    );
    delegate_rng!(
        u32,
        u32,
        impl RangeBounds<u32>,
        "Delegated [`Rng::u32`] method from [`Rng`]."
    );
    delegate_rng!(
        u16,
        u16,
        impl RangeBounds<u16>,
        "Delegated [`Rng::u16`] method from [`Rng`]."
    );
    delegate_rng!(
        u8,
        u8,
        impl RangeBounds<u8>,
        "Delegated [`Rng::u8`] method from [`Rng`]."
    );
    delegate_rng!(
        i128,
        i128,
        impl RangeBounds<i128>,
        "Delegated [`Rng::i128`] method from [`Rng`]."
    );
    delegate_rng!(
        i64,
        i64,
        impl RangeBounds<i64>,
        "Delegated [`Rng::i64`] method from [`Rng`]."
    );
    delegate_rng!(
        i32,
        i32,
        impl RangeBounds<i32>,
        "Delegated [`Rng::i32`] method from [`Rng`]."
    );
    delegate_rng!(
        i16,
        i16,
        impl RangeBounds<i16>,
        "Delegated [`Rng::i16`] method from [`Rng`]."
    );
    delegate_rng!(
        i8,
        i8,
        impl RangeBounds<i8>,
        "Delegated [`Rng::i8`] method from [`Rng`]."
    );
    delegate_rng!(
        usize,
        usize,
        impl RangeBounds<usize>,
        "Delegated [`Rng::usize`] method from [`Rng`]."
    );
    delegate_rng!(
        isize,
        isize,
        impl RangeBounds<isize>,
        "Delegated [`Rng::isize`] method from [`Rng`]."
    );
    delegate_rng!(
        chance,
        bool,
        f64,
        "Delegated [`Rng::chance`] method from [`Rng`]."
    );
    delegate_rng!(
        char,
        char,
        impl RangeBounds<char>,
        "Delegated [`Rng::char`] method from [`Rng`]."
    );
    delegate_rng!(
        digit,
        char,
        u8,
        "Delegated [`Rng::digit`] method from [`Rng`]."
    );
    delegate_rng!(
        alphabetic,
        char,
        "Delegated [`Rng::alphabetic`] method from [`Rng`]."
    );
    delegate_rng!(
        alphanumeric,
        char,
        "Delegated [`Rng::alphanumeric`] method from [`Rng`]."
    );
    delegate_rng!(
        lowercase,
        char,
        "Delegated [`Rng::lowercase`] method from [`Rng`]."
    );
    delegate_rng!(
        uppercase,
        char,
        "Delegated [`Rng::uppercase`] method from [`Rng`]."
    );
    delegate_rng!(bool, bool, "Delegated [`Rng::bool`] method from [`Rng`].");
    delegate_rng!(f64, f64, "Delegated [`Rng::f64`] method from [`Rng`].");
    delegate_rng!(f32, f32, "Delegated [`Rng::f32`] method from [`Rng`].");
    delegate_rng!(
        f64_normalized,
        f64,
        "Delegated [`Rng::f64_normalized`] method from [`Rng`]."
    );
    delegate_rng!(
        f32_normalized,
        f32,
        "Delegated [`Rng::f32_normalized`] method from [`Rng`]."
    );

    /// Delegated [`Rng::fill_bytes`] method from [`Rng`].
    #[inline]
    pub fn fill_bytes<B: AsMut<[u8]>>(&mut self, buffer: B) {
        self.get_mut().fill_bytes(buffer);
    }

    /// Delegated [`Rng::shuffle`] method from [`Rng`].
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        self.get_mut().shuffle(slice);
    }

    /// Delegated [`Rng::sample`] method from [`Rng`].
    #[inline]
    pub fn sample<'a, T>(&mut self, list: &'a [T]) -> Option<&'a T> {
        self.get_mut().sample(list)
    }

    /// Delegated [`Rng::sample_multiple`] method from [`Rng`].
    #[inline]
    pub fn sample_multiple<'a, T>(&mut self, list: &'a [T], amount: usize) -> Vec<&'a T> {
        self.get_mut().sample_multiple(list, amount)
    }

    /// Delegated [`Rng::weighted_sample`] method from [`Rng`].
    #[inline]
    pub fn weighted_sample<'a, T, F>(&mut self, list: &'a [T], weight_sampler: F) -> Option<&'a T>
    where
        F: Fn(&'a T) -> f64,
    {
        self.get_mut().weighted_sample(list, weight_sampler)
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
