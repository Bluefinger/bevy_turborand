#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_turborand::*;

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

/// A system for enemies attacking the player, applying randomised damage if they are able to land a hit.
fn attack_player(
    mut q_player: Query<&mut HitPoints, (With<Player>, Without<Enemy>)>,
    mut q_enemies: Query<(&Attack, &mut RngComponent), (With<Enemy>, Without<Player>)>,
) {
    let mut player = q_player.single_mut();

    for (attack, mut rng) in q_enemies.iter_mut() {
        let rng = rng.get_mut();

        if rng.chance(attack.hit) {
            player.total = player.total.saturating_sub(rng.u32(attack.min..=attack.max));
        }
    }
}

/// A system for seeing if the player will apply an attack on a random enemy or miss if unlucky!
fn attack_random_enemy(
    mut q_enemies: Query<&mut HitPoints, (With<Enemy>, Without<Player>)>,
    mut q_player: Query<(&Attack, &mut RngComponent), (With<Player>, Without<Enemy>)>,
) {
    let (attack, mut rng) = q_player.single_mut();

    let rng = rng.get_mut();

    for mut enemy in q_enemies.iter_mut() {
        if rng.chance(attack.hit) {
            enemy.total = enemy.total.saturating_sub(rng.u32(attack.min..=attack.max));
            break;
        }
    }
}

/// A system to randomly apply a healing effect on the player.
fn buff_player(
    mut q_player: Query<(&mut HitPoints, &mut RngComponent, &Buff), With<Player>>,
) {
    let (mut player, mut rng, buff) = q_player.single_mut();

    let rng = rng.get_mut();

    if rng.chance(buff.chance) {
        player.total = player.total.saturating_add(rng.u32(buff.min..=buff.max)).clamp(0, player.max);
    }
}

#[test]
fn deterministic_play_through() {
    // Set up the game App and World
    let mut app = App::new();

    let world = &mut app.world;

    // Initialise our global Rng resource
    let mut global_rng = GlobalRng::new(Some(123456));

    // Spawn the player
    let mut player = world.spawn();
    let player_id = player
        .insert(Player)
        .insert(HitPoints {
            total: 100,
            max: 100
        })
        .insert(Attack {
            min: 5,
            max: 10,
            hit: 0.6,
        })
        .insert(Buff {
            min: 2,
            max: 6,
            chance: 0.10,
        })
        .insert(RngComponent::from_global(&mut global_rng))
        .id();

    // Spawn some enemies for the player to fight with
    let mut enemy_1 = world.spawn();
    let enemy_1_id = enemy_1
        .insert(Enemy)
        .insert(HitPoints {
            total: 20,
            max: 20,
        })
        .insert(Attack {
            min: 3,
            max: 6,
            hit: 0.5,
        })
        .insert(RngComponent::from_global(&mut global_rng))
        .id();

    let mut enemy_2 = world.spawn();
    let enemy_2_id = enemy_2
        .insert(Enemy)
        .insert(HitPoints {
            total: 20,
            max: 20,
        })
        .insert(Attack {
            min: 3,
            max: 6,
            hit: 0.5,
        })
        .insert(RngComponent::from_global(&mut global_rng))
        .id();

    // Add the systems to our App. Order the necessary systems in order
    // to ensure deterministic behaviour.
    app.add_system(attack_player);
    app.add_system(attack_random_enemy);
    app.add_system(buff_player.after(attack_random_enemy));

    // Run the game once!
    app.update();

    // Check to see the health of our combatants, looks like player missed his attacks...
    assert_eq!(app.world.get::<HitPoints>(player_id).unwrap().total, 100);
    assert_eq!(app.world.get::<HitPoints>(enemy_1_id).unwrap().total, 20);
    assert_eq!(app.world.get::<HitPoints>(enemy_2_id).unwrap().total, 20);

    // Again!
    app.update();

    // Player OP, nerf heals.
    assert_eq!(app.world.get::<HitPoints>(player_id).unwrap().total, 100);
    assert_eq!(app.world.get::<HitPoints>(enemy_1_id).unwrap().total, 13);
    assert_eq!(app.world.get::<HitPoints>(enemy_2_id).unwrap().total, 20);

    // And again!
    app.update();

    // Enemies are in trouble...
    assert_eq!(app.world.get::<HitPoints>(player_id).unwrap().total, 96);
    assert_eq!(app.world.get::<HitPoints>(enemy_1_id).unwrap().total, 13);
    assert_eq!(app.world.get::<HitPoints>(enemy_2_id).unwrap().total, 12);
}
