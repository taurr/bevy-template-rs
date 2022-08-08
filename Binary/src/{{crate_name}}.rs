use bevy::prelude::*;

pub struct {{crate_name|pascal_case}}Plugin;

impl Plugin for {{crate_name|pascal_case}}Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(rotate);
    }
}

#[derive(Debug, Component, Default)]
struct {{crate_name|pascal_case}}Component;

#[derive(Bundle, Default)]
struct {{crate_name|pascal_case}}Bundle {
    pub example: {{crate_name|pascal_case}}Component,
    {% if camera=="3D" -%}
    #[bundle]
    pub scene: SceneBundle,
    {%- else -%}
    #[bundle]
    pub spatial: SpatialBundle,
    {%- endif %}
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    {% if camera=="3D" -%}
    use std::f32::consts::TAU;
    commands
        .spawn_bundle({{crate_name|pascal_case}}Bundle {
            scene: SceneBundle {
                scene: asset_server.load("cube.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    22.5 * TAU / 360.0,
                    45.0 * TAU / 360.0,
                    0.0,
                )),
                ..default()
            },
            ..default()
        });
    {%- else -%}
    use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
    commands
        .spawn_bundle({{crate_name|pascal_case}}Bundle::default())
        .with_children(|commands| {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(VIEWPORT_WIDTH / 2.0, VIEWPORT_HEIGHT / 2.0)),
                    ..Default::default()
                },
                texture: asset_server.load("icon.png"),
                ..Default::default()
            });
        });
    {%- endif %}
}

fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<{{crate_name|pascal_case}}Component>>) {
    use std::f32::consts::TAU;
    for mut transform in &mut transforms {
        transform.rotate_z(45.0 * TAU / 360.0 * time.delta_seconds());
    }
}
