#![doc = include_str!("../README.md")]

{% if camera=="2D" -%}
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
{%- else -%}
use bevy::prelude::*;
{%- endif %}

mod debug;
mod {{crate_name}};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
{% if camera=="2D" -%}
const VIEWPORT_WIDTH: f32 = 80.0;
const VIEWPORT_HEIGHT: f32 = 45.0;
{%- endif -%}
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!(
                "{} - v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        {% if camera=="3D" -%}
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        })
        {%- else -%}
        .insert_resource(ImageSettings::default_nearest())
        {%- endif %}
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin({{crate_name}}::{{crate_name|pascal_case}}Plugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    {% if camera=="3D" -%}
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    {%- else -%}
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Auto {
                min_width: VIEWPORT_WIDTH,
                min_height: VIEWPORT_HEIGHT,
            },
            ..default()
        },
        ..default()
    });
    {%- endif %}
}
