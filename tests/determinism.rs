#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_turborand::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Component, Default)]
struct HitPoints {
    total: u32,
    max: u32,
}

#[derive(Debug, Component, Default)]
struct Attack {
    min: u32,
    max: u32,
    hit: f64,
}

#[derive(Debug, Component, Default)]
struct Buff {
    min: u32,
    max: u32,
    chance: f64,
}

#[derive(Debug, Component, Default)]
struct Player;
#[derive(Debug, Component, Default)]
struct Enemy;

fn setup_player(mut commands: Commands, mut global: ResMut<GlobalRng>) {
    commands.spawn((Player, RngComponent::from(&mut global)));
}

fn setup_enemies(mut commands: Commands, mut global: ResMut<GlobalRng>) {
    for _ in 0..2 {
        commands.spawn((Enemy, RngComponent::from(&mut global)));
    }
}

#[cfg(feature = "chacha")]
fn setup_secure_player(mut commands: Commands, mut global: ResMut<GlobalChaChaRng>) {
    commands.spawn((Player, ChaChaRngComponent::from(&mut global)));
}

#[cfg(feature = "chacha")]
fn setup_secure_enemies(mut commands: Commands, mut global: ResMut<GlobalChaChaRng>) {
    for _ in 0..2 {
        commands.spawn((Enemy, ChaChaRngComponent::from(&mut global)));
    }
}

/// A system for enemies attacking the player, applying randomised damage if they are able to land a hit.
fn attack_player(
    mut q_player: Query<&mut HitPoints, (With<Player>, Without<Enemy>)>,
    mut q_enemies: Query<(&Attack, &mut RngComponent), (With<Enemy>, Without<Player>)>,
) {
    let mut player = q_player.single_mut();

    for (attack, mut rng) in q_enemies.iter_mut() {
        if rng.chance(attack.hit) {
            player.total = player
                .total
                .saturating_sub(rng.u32(attack.min..=attack.max));
        }
    }
}

/// A system for seeing if the player will apply an attack on a random enemy or miss if unlucky!
fn attack_random_enemy(
    mut q_enemies: Query<&mut HitPoints, (With<Enemy>, Without<Player>)>,
    mut q_player: Query<(&Attack, &mut RngComponent), (With<Player>, Without<Enemy>)>,
) {
    let (attack, mut rng) = q_player.single_mut();

    for mut enemy in q_enemies.iter_mut() {
        if rng.chance(attack.hit) {
            enemy.total = enemy.total.saturating_sub(rng.u32(attack.min..=attack.max));
            break;
        }
    }
}

/// A system to randomly apply a healing effect on the player.
fn buff_player(mut q_player: Query<(&mut HitPoints, &mut RngComponent, &Buff), With<Player>>) {
    let (mut player, mut rng, buff) = q_player.single_mut();

    if rng.chance(buff.chance) {
        player.total = player
            .total
            .saturating_add(rng.u32(buff.min..=buff.max))
            .clamp(0, player.max);
    }
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn deterministic_play_through() {
    // Set up the game App and World
    let mut app = App::new();

    let world = app.world_mut();

    // Initialise our global Rng resource
    let mut global_rng = GlobalRng::with_seed(12345);

    // Spawn the player
    let player = world
        .spawn((
            Player,
            HitPoints {
                total: 100,
                max: 100,
            },
            Attack {
                min: 5,
                max: 10,
                hit: 0.6,
            },
            Buff {
                min: 2,
                max: 6,
                chance: 0.10,
            },
            RngComponent::from(&mut global_rng),
        ))
        .id();

    // Spawn some enemies for the player to fight with
    let enemy_1 = world
        .spawn((
            Enemy,
            HitPoints { total: 20, max: 20 },
            Attack {
                min: 3,
                max: 6,
                hit: 0.5,
            },
            RngComponent::from(&mut global_rng),
        ))
        .id();

    let enemy_2 = world
        .spawn((
            Enemy,
            HitPoints { total: 20, max: 20 },
            Attack {
                min: 3,
                max: 6,
                hit: 0.5,
            },
            RngComponent::from(&mut global_rng),
        ))
        .id();

    // Add the systems to our App. Order the necessary systems in order
    // to ensure deterministic behaviour.
    app.add_systems(
        Update,
        ((attack_random_enemy, buff_player).chain(), attack_player),
    );

    // Run the game once!
    app.update();

    // Check to see the health of our combatants
    assert_eq!(app.world().get::<HitPoints>(player).unwrap().total, 100);
    assert_eq!(app.world().get::<HitPoints>(enemy_1).unwrap().total, 20);
    assert_eq!(app.world().get::<HitPoints>(enemy_2).unwrap().total, 11);

    // Again!
    app.update();

    // Player OP. Enemy 2 is in trouble
    assert_eq!(app.world().get::<HitPoints>(player).unwrap().total, 90);
    assert_eq!(app.world().get::<HitPoints>(enemy_1).unwrap().total, 20);
    assert_eq!(app.world().get::<HitPoints>(enemy_2).unwrap().total, 3);

    // And again!
    app.update();

    // Enemy 2 is now deceased
    assert_eq!(app.world().get::<HitPoints>(player).unwrap().total, 88);
    assert_eq!(app.world().get::<HitPoints>(enemy_1).unwrap().total, 20);
    assert_eq!(app.world().get::<HitPoints>(enemy_2).unwrap().total, 0);
}

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn deterministic_setup() {
    let mut app = App::new();

    app.insert_resource(GlobalRng::with_seed(23456));

    app.add_systems(Startup, (setup_player, setup_enemies).chain());

    app.update();

    let mut q_player = app
        .world_mut()
        .query_filtered::<&mut RngComponent, With<Player>>();
    let mut player = q_player.single_mut(app.world_mut());

    assert_eq!(player.u32(..=10), 10);

    let mut q_enemies = app
        .world_mut()
        .query_filtered::<&mut RngComponent, With<Enemy>>();
    let mut enemies = q_enemies.iter_mut(app.world_mut());

    let mut enemy_1 = enemies.next().unwrap();

    assert_eq!(enemy_1.u32(..=10), 1);

    let mut enemy_2 = enemies.next().unwrap();

    assert_eq!(enemy_2.u32(..=10), 7);
}

#[cfg(feature = "chacha")]
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn deterministic_secure_setup() {
    let mut app = App::new();

    app.insert_resource(GlobalChaChaRng::with_seed([1; 40]));

    app.add_systems(Startup, (setup_secure_player, setup_secure_enemies).chain());

    app.update();

    let mut q_player = app
        .world_mut()
        .query_filtered::<&mut ChaChaRngComponent, With<Player>>();
    let mut player = q_player.single_mut(app.world_mut());

    assert_eq!(player.u32(..=10), 0);

    let mut q_enemies = app
        .world_mut()
        .query_filtered::<&mut ChaChaRngComponent, With<Enemy>>();
    let mut enemies = q_enemies.iter_mut(app.world_mut());

    let mut enemy_1 = enemies.next().unwrap();

    assert_eq!(enemy_1.u32(..=10), 6);

    let mut enemy_2 = enemies.next().unwrap();

    assert_eq!(enemy_2.u32(..=10), 3);
}

#[cfg(feature = "serialize")]
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn load_rng_setup() {
    let payload = "(((state:(24691))))";

    let mut rng: RngComponent = ron::from_str(payload).unwrap();

    assert_eq!(rng.u32(..10), 4);
}

#[cfg(all(feature = "serialize", feature = "chacha"))]
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn load_chacha_rng_setup() {
    let payload = "(((state:(1634760805,857760878,2036477234,1797285236,117901063,117901063,117901063,117901063,117901063,117901063,117901063,117901063,0,0,117901063,117901063),cache:(0,0,0,0,0,0,0,0,64))))";

    let mut rng: ChaChaRngComponent = ron::from_str(payload).unwrap();

    assert_eq!(rng.u32(..10), 0);
}

#[cfg(feature = "serialize")]
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn rng_reflection() {
    use bevy::reflect::{
        serde::{ReflectDeserializer, ReflectSerializer},
        TypeRegistry,
    };
    use ron::ser::to_string;
    use serde::de::DeserializeSeed;

    let mut registry = TypeRegistry::default();
    registry.register::<RngComponent>();

    let mut val = RngComponent::with_seed(7);

    let ser = ReflectSerializer::new(&val, &registry);

    let serialized = to_string(&ser).unwrap();

    assert_eq!(
        &serialized,
        "{\"bevy_turborand::component::rng::RngComponent\":(((state:(15))))}"
    );

    let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

    let de = ReflectDeserializer::new(&registry);

    let value = de.deserialize(&mut deserializer).unwrap();

    let mut dynamic = RngComponent::take_from_reflect(value).unwrap();

    assert_eq!(val.get_mut(), dynamic.get_mut());

    dynamic.u64(..);

    assert_ne!(val.get_mut(), dynamic.get_mut());
}

#[cfg(all(feature = "serialize", feature = "chacha"))]
#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn chacha_rng_reflection() {
    use bevy::reflect::{
        serde::{ReflectDeserializer, ReflectSerializer},
        TypeRegistry,
    };
    use serde::de::DeserializeSeed;

    let mut registry = TypeRegistry::default();
    registry.register::<ChaChaRngComponent>();

    let mut val = ChaChaRngComponent::with_seed([7; 40]);

    let ser = ReflectSerializer::new(&val, &registry);

    let serialized = ron::ser::to_string_pretty(
        &ser,
        ron::ser::PrettyConfig::new().new_line(String::from("\n")),
    )
    .unwrap();

    assert_eq!(
        &serialized,
        r#"{
    "bevy_turborand::component::chacha::ChaChaRngComponent": (((
        state: (1634760805, 857760878, 2036477234, 1797285236, 117901063, 117901063, 117901063, 117901063, 117901063, 117901063, 117901063, 117901063, 0, 0, 117901063, 117901063),
        cache: (0, 0, 0, 0, 0, 0, 0, 0, 64),
    ))),
}"#
    );

    let mut deserializer = ron::Deserializer::from_str(&serialized).unwrap();

    let de = ReflectDeserializer::new(&registry);

    let value = de.deserialize(&mut deserializer).unwrap();

    let mut dynamic = ChaChaRngComponent::take_from_reflect(value).unwrap();

    assert_eq!(val.get_mut(), dynamic.get_mut());

    dynamic.u64(..);

    assert_ne!(val.get_mut(), dynamic.get_mut());
}
