use crate::*;

/// A [`SecureRng`] component that wraps a random number generator,
/// specifically the [`SecureRng`] struct, which provides a cryptographically
/// secure source based on ChaCha8.
///
/// # Creating new [`SecureRngComponent`]s.
///
/// You can creates a new  [`SecureRngComponent`] directly from anything that yields
/// a mut reference to a [`DelegatedRng`], such as [`ResMut`], or a
/// [`Component`], or from a [`TurboCore`] source directly. You can't create or seed
/// [`SecureRngComponent`] from sources that are not backed by a [`SecureCore`] source.
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
/// fn setup_source(mut commands: Commands, mut global: ResMut<GlobalSecureRng>) {
///     commands
///         .spawn()
///         .insert(Source)
///         .insert(SecureRngComponent::from(&mut global));
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
///    mut q_source: Query<&mut SecureRngComponent, (With<Source>, Without<Enemy>)>,
/// ) {
///    let mut source = q_source.single_mut();
///
///    for _ in 0..2 {
///        commands
///            .spawn()
///            .insert(Enemy)
///            .insert(SecureRngComponent::from(&mut source));
///    }
/// }
/// ```
#[derive(Debug, Component)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct SecureRngComponent(SecureRng);

unsafe impl Sync for SecureRngComponent {}

impl SecureRngComponent {
    /// Create a new [`SecureRngComponent`] with a randomised seed.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(SecureRng::new())
    }

    /// Create a new [`SecureRngComponent`] with a given seed.
    #[inline]
    #[must_use]
    pub fn with_seed(seed: [u8; 40]) -> Self {
        Self(SecureRng::with_seed(seed))
    }
}

impl DelegatedRng for SecureRngComponent {
    type Source = SecureRng;

    #[inline]
    fn get_mut(&mut self) -> &mut Self::Source {
        &mut self.0
    }
}

impl Default for SecureRngComponent {
    /// Creates a default [`SecureRngComponent`] instance. The instance will
    /// be initialised with a randomised seed, so this is **not**
    /// deterministic.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: TurboCore + SecureCore> From<&T> for SecureRngComponent {
    #[inline]
    #[must_use]
    fn from(rng: &T) -> Self {
        Self(SecureRng::with_seed(rng.gen::<40>()))
    }
}

impl<T: DelegatedRng> From<&mut T> for SecureRngComponent
where
    <T as DelegatedRng>::Source: SecureCore,
{
    #[inline]
    #[must_use]
    fn from(rng: &mut T) -> Self {
        Self(SecureRng::with_seed(rng.get_mut().gen::<40>()))
    }
}

impl<T: DelegatedRng> From<&mut Mut<'_, T>> for SecureRngComponent
where
    <T as DelegatedRng>::Source: SecureCore,
{
    #[inline]
    #[must_use]
    fn from(rng: &mut Mut<'_, T>) -> Self {
        Self(SecureRng::with_seed(rng.get_mut().gen::<40>()))
    }
}

impl<T: DelegatedRng + Send + Sync + 'static> From<&mut ResMut<'_, T>> for SecureRngComponent
where
    <T as DelegatedRng>::Source: SecureCore,
{
    #[inline]
    #[must_use]
    fn from(rng: &mut ResMut<'_, T>) -> Self {
        Self(SecureRng::with_seed(rng.get_mut().gen::<40>()))
    }
}
