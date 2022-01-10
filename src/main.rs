use anyhow::Result;
use bevy::prelude::*;

use {{crate_name}}::{*, debug::BevyDebug};

mod asset_io;

fn main() -> Result<()> {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Example window".into(),
        mode: bevy::window::WindowMode::Windowed,
        width: 1280.,
        height: 600.,
        resizable: false,
        scale_factor_override: Some(1.0),
        ..Default::default()
    })
    .add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(asset_io::InMemoryAssetPlugin)
    })
    .add_plugin(HelloPlugin)
    .add_inspector()
    .write_graphs()?
    .run();
    Ok(())
}
