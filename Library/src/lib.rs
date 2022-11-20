#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub struct {{crate_name|pascal_case}}Plugin;

#[derive(Component, Default)]
pub struct Enemy {
    pub hit_points: u32,
}

impl Plugin for {{crate_name|pascal_case}}Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::despawn_dead_enemies)
            .add_system(Self::hurt_enemies.before(Self::despawn_dead_enemies));
    }
}

impl {{crate_name|pascal_case}}Plugin {
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
}
