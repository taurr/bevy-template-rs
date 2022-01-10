use bevy::prelude::*;

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
    // Setup world
    let mut world = World::default();

    // Setup stage with our two systems
    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(hurt_enemies{%if bevy_version!="git"%}.system(){%endif%}.before("death"));
    update_stage.add_system(despawn_dead_enemies{%if bevy_version!="git"%}.system(){%endif%}.label("death"));

    // Setup test entities
    let enemy_id = world.spawn().insert(Enemy { hit_points: 5 }).id();

    // Run systems
    update_stage.run(&mut world);

    // Check resulting changes
    assert!(world.get::<Enemy>(enemy_id).is_some());
    assert_eq!(world.get::<Enemy>(enemy_id).unwrap().hit_points, 4);
}
