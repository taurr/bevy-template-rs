use anyhow::Result;
use bevy::{prelude::*, window::{PresentMode, WindowMode}};

mod debug;

use {{crate_name}}::HelloPlugin;

const HEIGHT: f32 = 900.0;
const RESOLUTION: f32 = 16.0/9.0;
const BACKGROUND: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() -> Result<()> {
    let mut app = App::new();
    app.insert_resource(ClearColor(BACKGROUND))
        .insert_resource(WindowDescriptor {
            title: format!("{} - v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(debug::DebugPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(setup)
        .run();
    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
