use crate::*;

/// A [`Rng`] component that wraps a random number generator,
/// specifically the [`Rng`] struct, which provides a fast, but
/// not cryptographically secure source based on WyRand.
///
/// # Creating new [`RngComponent`]s.
///
/// You can creates a new [`RngComponent`] directly from anything that yields
/// a mut reference to a [`DelegatedRng`], such as [`ResMut`] or a
/// [`Component`], or from a [`TurboCore`] source directly.
///
/// # Examples
///
/// From a resource:
/// ```
/// use bevy::prelude::*;
/// use bevy_turborand::*;
///
/// #[derive(Debug, Component, Default)]
/// struct Source;
///
/// fn setup_source(mut commands: Commands, mut global: ResMut<GlobalRng>) {
///     commands
///         .spawn((
///             Source,
///             RngComponent::from(&mut global),
///         ));
/// }
/// ```
///
/// From a component:
/// ```
/// use bevy::prelude::*;
/// use bevy_turborand::*;
///
/// #[derive(Debug, Component, Default)]
/// struct Enemy;
/// #[derive(Debug, Component, Default)]
/// struct Source;
///
/// fn setup_enemies_from_source(
///    mut commands: Commands,
///    mut q_source: Query<&mut RngComponent, (With<Source>, Without<Enemy>)>,
/// ) {
///    let mut source = q_source.single_mut();
///
///    for _ in 0..2 {
///        commands
///            .spawn((
///                Enemy,
///                RngComponent::from(&mut source)
///            ));
///    }
/// }
/// ```
#[derive(Debug, Clone, Component)]
#[cfg_attr(docsrs, doc(cfg(feature = "wyrand")))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct RngComponent(Rng);

unsafe impl Sync for RngComponent {}

impl RngComponent {
    /// Create a new [`RngComponent`] with a randomised seed.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(Rng::new())
    }

    /// Create a new [`RngComponent`] instance with a given seed.
    #[inline]
    #[must_use]
    pub fn with_seed(seed: u64) -> Self {
        Self(Rng::with_seed(seed))
    }
}

impl DelegatedRng for RngComponent {
    type Source = Rng;

    #[inline]
    #[must_use]
    fn get_mut(&mut self) -> &mut Self::Source {
        &mut self.0
    }
}

impl Default for RngComponent {
    /// Creates a default [`RngComponent`] instance. The instance will
    /// be initialised with a randomised seed, so this is **not**
    /// deterministic.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TurboCore + GenCore> From<&T> for RngComponent {
    #[inline]
    #[must_use]
    fn from(rng: &T) -> Self {
        Self(Rng::with_seed(rng.gen_u64()))
    }
}

impl<T: DelegatedRng> From<&mut T> for RngComponent {
    #[inline]
    #[must_use]
    fn from(rng: &mut T) -> Self {
        Self(Rng::with_seed(rng.get_mut().gen_u64()))
    }
}

impl<T: DelegatedRng> From<&mut Mut<'_, T>> for RngComponent {
    #[inline]
    #[must_use]
    fn from(rng: &mut Mut<'_, T>) -> Self {
        Self(Rng::with_seed(rng.get_mut().gen_u64()))
    }
}

impl<T: DelegatedRng + Resource + Send + Sync + 'static> From<&mut ResMut<'_, T>> for RngComponent {
    #[inline]
    #[must_use]
    fn from(rng: &mut ResMut<'_, T>) -> Self {
        Self(Rng::with_seed(rng.get_mut().gen_u64()))
    }
}
