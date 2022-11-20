#![doc = include_str!("../README.md")]

{%- if camera=="2D" %}
use bevy::{prelude::*, render::camera::ScalingMode};
{%- else -%}
use bevy::prelude::*;
{%- endif %}
use {{crate_name}}::*;

mod debug;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: format!(
                    "{} - v{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ),
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugins(debug::DebugPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin({{crate_name|pascal_case}}Plugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_web_resizer::Plugin);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    {% if camera=="3D" -%}
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    {%- else -%}
    commands.spawn(Camera2dBundle {
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
