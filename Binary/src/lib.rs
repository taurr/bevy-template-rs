use bevy::prelude::*;

{% if camera=="2D" -%}
pub const VIEWPORT_WIDTH: f32 = 80.0;
pub const VIEWPORT_HEIGHT: f32 = 80.0;

{% endif -%}pub struct {{crate_name|pascal_case}}Plugin;

impl Plugin for {{crate_name|pascal_case}}Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup).add_system(Self::rotate);
    }
}

#[derive(Debug, Component, Default)]
pub struct {{crate_name|pascal_case}}Component;

impl {{crate_name|pascal_case}}Plugin {
    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        {% if camera=="3D" -%}
        use std::f32::consts::TAU;
        commands
            .spawn({{crate_name|pascal_case}}Component)
            .insert(Name::new("{{crate_name|pascal_case}}Plugin Root"))
            .insert(SpatialBundle::default())
            .with_children(|commands| {
                commands
                    .spawn(SceneBundle {
                        scene: asset_server.load("cube.glb#Scene0"),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(
                            Quat::from_euler(
                                EulerRot::XYZ,
                                22.5 * TAU / 360.0,
                                45.0 * TAU / 360.0,
                                0.0,
                            ),
                        ),
                        ..default()
                    })
                    .insert(Name::new("{{crate_name|pascal_case}}Plugin Scene"));
            });
        {%- else -%}
        commands
            .spawn({{crate_name|pascal_case}}Component)
            .insert(Name::new("{{crate_name|pascal_case}}Plugin Root"))
            .insert(SpatialBundle::default())
            .with_children(|commands| {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(
                                VIEWPORT_WIDTH / 2.0,
                                VIEWPORT_HEIGHT / 2.0,
                            )),
                            ..default()
                        },
                        texture: asset_server.load("icon.png"),
                        ..default()
                })
                .insert(Name::new("{{crate_name|pascal_case}}Plugin Sprite"));
            });
        {%- endif %}
    }

    fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<{{crate_name|pascal_case}}Component>>) {
        use std::f32::consts::TAU;
        for mut transform in &mut transforms {
            transform.rotate_z(45.0 * TAU / 360.0 * time.delta_seconds());
        }
    }
}
