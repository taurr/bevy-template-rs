use std::f32::consts::TAU;

use bevy::prelude::*;

pub struct {{crate_name|pascal_case}}Plugin;

#[derive(Debug, Component)]
struct ExampleObject;

impl Plugin for {{crate_name|pascal_case}}Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup).add_system(Self::rotate);
    }
}

impl {{crate_name|pascal_case}}Plugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        {% if camera=="3D" %}commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        });
        commands
            .spawn_bundle(SceneBundle {
                scene: asset_server.load("cube.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    22.5 * TAU / 360.0,
                    45.0 * TAU / 360.0,
                    0.0,
                )),
                ..default()
            })
            .insert(ExampleObject);{% else %}commands.spawn_bundle(Camera2dBundle::default());
        commands
            .spawn()
            .insert(ExampleObject)
            .insert_bundle(SpatialBundle::default())
            .with_children(|commands| {
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..Default::default()
                    },
                    texture: asset_server.load("icon.png"),
                    ..Default::default()
                });
            });{% endif %}
    }

    fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<ExampleObject>>) {
        for mut transform in transforms.iter_mut() {
            transform.rotate_z(45.0 * TAU / 360.0 * time.delta_seconds());
        }
    }
}
