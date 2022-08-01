#![doc = include_str!("../README.md")]

{% if camera=="3D" %}use std::f32::consts::TAU;

{% endif %}use anyhow::Result;
use bevy::{prelude::*, window::WindowMode};

const HEIGHT: f32 = 900.0;
const ASPECT_RATIO: f32 = 16.0/9.0;
const BACKGROUND: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() -> Result<()> {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!(
                "{} - v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            width: HEIGHT * ASPECT_RATIO,
            height: HEIGHT,
            mode: WindowMode::Windowed,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND)){% if camera=="3D" %}
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.2,
        }){% endif %}
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugins)
        .add_startup_system(setup)
        .run();
    Ok(())
}

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
        .insert(Name::new("Scene"));{% else %}commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..Default::default()
        },
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });{% endif %}
}

struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    #[cfg(not(feature = "editor"))]
    fn build(&mut self, _: &mut bevy::app::PluginGroupBuilder) {
    }

    #[cfg(feature = "editor")]
    fn build(&mut self, #[allow(unused)] group: &mut bevy::app::PluginGroupBuilder) {
        use bevy_editor_pls::EditorPlugin;
        group
            .add(EditorPlugin)
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add(bevy::diagnostic::EntityCountDiagnosticsPlugin);
    }
}
