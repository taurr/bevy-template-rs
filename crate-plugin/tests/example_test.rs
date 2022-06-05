use bevy::prelude::*;

mod common;

#[derive(Component, Default)]
struct Enemy {
    hit_points: u32,
}

fn despawn_dead_enemies(mut commands: Commands, enemies: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in enemies.iter() {
        if enemy.hit_points == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn hurt_enemies(mut enemies: Query<&mut Enemy>) {
    for mut enemy in enemies.iter_mut() {
        enemy.hit_points -= 1;
    }
}

#[test]
fn did_hurt_enemy() {
    let mut app = common::minimal_bevy_app();

    app.add_system(despawn_dead_enemies)
        .add_system(hurt_enemies.before(despawn_dead_enemies));

    // Setup test entities
    let enemy_id = app.world.spawn().insert(Enemy { hit_points: 5 }).id();

    // spin Bevy a few times...
    (0..1).for_each(|_| app.update());

    // Check resulting changes
    assert!(app.world.get::<Enemy>(enemy_id).is_some());
    assert_eq!(app.world.get::<Enemy>(enemy_id).unwrap().hit_points, 4);
}
