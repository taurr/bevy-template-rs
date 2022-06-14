use anyhow::Result;
use bevy::{prelude::*, window::WindowMode};

const HEIGHT: f32 = 900.0;
const ASPECT_RATIO: f32 = 16.0/9.0;
const BACKGROUND: Color = Color::rgb(0.1, 0.1, 0.1);

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
