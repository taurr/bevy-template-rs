mod debug;

use anyhow::Result;
use bevy::prelude::*;

#[cfg(any(feature = "egui_inspector", feature = "write_graphs"))]
use debug::BevyDebug;

use {{crate_name}}::HelloPlugin;

fn main() -> Result<()> {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.039, 0.055, 0.078)))
        .insert_resource(WindowDescriptor {
            title: "Example window".into(),
            mode: bevy::window::WindowMode::Windowed,
            width: 800.,
            height: 600.,
            resizable: false,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .add_plugin(HelloPlugin)
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "egui_inspector")]
    app.add_world_inspector()
        .add_inspector::<bevy_inspector_egui::widgets::InspectorQuery<Entity, With<{{crate_name}}::Person>>>();

    #[cfg(feature = "write_graphs")]
    app.write_graphs()?;

    app.run();
    Ok(())
}
