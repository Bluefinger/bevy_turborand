use crate::*;

/// A [`ChaChaRng`] component that wraps a random number generator,
/// specifically the [`ChaChaRng`] struct, which provides a cryptographically
/// secure source based on ChaCha8.
///
/// # Creating new [`ChaChaRngComponent`]s.
///
/// You can creates a new  [`ChaChaRngComponent`] directly from anything that yields
/// a mut reference to a [`DelegatedRng`], such as [`ResMut`], or a
/// [`Component`], or from a [`TurboCore`] source directly. You can't create or seed
/// [`ChaChaRngComponent`] from sources that are not backed by a [`SecureCore`] source.
///
/// # Examples
///
/// From a resource:
/// ```
/// use bevy::prelude::*;
/// use bevy_turborand::prelude::*;
///
/// #[derive(Debug, Component, Default)]
/// struct Source;
///
/// fn setup_source(mut commands: Commands, mut global: ResMut<GlobalChaChaRng>) {
///     commands
///         .spawn((
///             Source,
///             ChaChaRngComponent::from(&mut global)
///         ));
/// }
/// ```
///
/// From a component:
/// ```
/// use bevy::prelude::*;
/// use bevy_turborand::prelude::*;
///
/// #[derive(Debug, Component, Default)]
/// struct Enemy;
/// #[derive(Debug, Component, Default)]
/// struct Source;
///
/// fn setup_enemies_from_source(
///    mut commands: Commands,
///    mut q_source: Query<&mut ChaChaRngComponent, (With<Source>, Without<Enemy>)>,
/// ) {
///    let mut source = q_source.single_mut().unwrap();
///
///    for _ in 0..2 {
///        commands
///            .spawn((
///                 Enemy,
///                 ChaChaRngComponent::from(&mut source),
///             ));
///    }
/// }
/// ```
#[derive(Debug, Clone, Component, PartialEq, Reflect)]
#[cfg_attr(docsrs, doc(cfg(feature = "chacha")))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serialize",
    reflect(opaque, Debug, PartialEq, Default, Serialize, Deserialize)
)]
#[cfg_attr(not(feature = "serialize"), reflect(opaque, Debug, PartialEq, Default))]
pub struct ChaChaRngComponent(ChaChaRng);

unsafe impl Sync for ChaChaRngComponent {}

impl ChaChaRngComponent {
    /// Create a new [`ChaChaRngComponent`] with a randomised seed.
    #[inline]
    pub fn new() -> Self {
        Self(ChaChaRng::new())
    }

    /// Create a new [`ChaChaRngComponent`] with a given seed.
    #[inline]
    pub fn with_seed(seed: [u8; 40]) -> Self {
        Self(ChaChaRng::with_seed(seed))
    }
}

impl DelegatedRng for ChaChaRngComponent {
    type Source = ChaChaRng;

    #[inline]
    fn get_mut(&mut self) -> &mut Self::Source {
        &mut self.0
    }
}

impl Default for ChaChaRngComponent {
    /// Creates a default [`ChaChaRngComponent`] instance. The instance will
    /// be initialised with a randomised seed, so this is **not**
    /// deterministic.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TurboCore + GenCore + SecureCore> From<&T> for ChaChaRngComponent {
    #[inline]
    fn from(rng: &T) -> Self {
        Self(ChaChaRng::with_seed(rng.r#gen()))
    }
}

impl<T: DelegatedRng> From<&mut T> for ChaChaRngComponent
where
    T::Source: SecureCore,
{
    #[inline]
    fn from(rng: &mut T) -> Self {
        Self(ChaChaRng::with_seed(rng.get_mut().r#gen()))
    }
}

impl<T: DelegatedRng> From<&mut Mut<'_, T>> for ChaChaRngComponent
where
    T::Source: SecureCore,
{
    #[inline]
    fn from(rng: &mut Mut<'_, T>) -> Self {
        Self(ChaChaRng::with_seed(rng.get_mut().r#gen()))
    }
}

impl<T: DelegatedRng + Resource + Send + Sync + 'static> From<&mut ResMut<'_, T>>
    for ChaChaRngComponent
where
    T::Source: SecureCore,
{
    #[inline]
    fn from(rng: &mut ResMut<'_, T>) -> Self {
        Self(ChaChaRng::with_seed(rng.get_mut().r#gen()))
    }
}
