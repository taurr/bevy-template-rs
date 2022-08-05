#![doc = include_str!("../README.md")]

use anyhow::Result;
use bevy::prelude::*;

mod debug;
mod {{crate_name}};

const HEIGHT: f32 = 480.0;
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
        .add_plugin({{crate_name}}::{{crate_name|pascal_case}}Plugin)
        .run();
    Ok(())
}
