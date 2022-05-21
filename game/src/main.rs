use anyhow::Result;
use bevy::{prelude::*, window::{PresentMode, WindowMode}};

mod debug;

use {{crate_name}}::HelloPlugin;

const HEIGHT: f32 = 900.0;
const RESOLUTION: f32 = 16.0/9.0;
const BACKGROUND: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() -> Result<()> {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!(
                "{} - v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND)){% if camera=="3D" %}
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        }){% endif %}
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_plugin(HelloPlugin){% if camera!="None" %}
        .add_startup_system(setup){% endif %}
        .run();
    Ok(())
}
{% if camera != "None" %}
fn setup(mut commands: Commands{% if camera == "3D" %}, asset_server: Res<AssetServer>{% endif %}) {
    {% if camera=="3D" %}commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(4., 2., -4.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Name::new("Scene"))
        .with_children(|b| {
            b.spawn_scene(asset_server.load("cube.glb#Scene0"));
        });{% else %}commands.spawn_bundle(OrthographicCameraBundle::new_2d());{% endif %}
}{% endif %}
