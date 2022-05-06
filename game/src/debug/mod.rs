use bevy::prelude::*;

#[cfg(feature = "inspector")]
mod inspector;

#[cfg(feature = "write_graphs")]
mod write_graphs;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    #[cfg(feature = "inspector")]
    fn build(&self, app: &mut App) {
        use inspector::*;
        app.add_world_inspector()
            .add_inspector::<InspectorQuery<Entity, With<{{crate_name}}::Person>>>();
    }

    #[cfg(not(feature = "inspector"))]
    fn build(&self, _app: &mut App) {
    }
}

pub struct GraphWriterPlugin;

impl Plugin for GraphWriterPlugin {
    #[cfg(feature = "write_graphs")]
    fn build(&self, app: &mut App) {
        use write_graphs::*;
        app.write_graphs()?;
    }

    #[cfg(not(feature = "write_graphs"))]
    fn build(&self, _app: &mut App) {
    }
}

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(InspectorPlugin)
        .add(GraphWriterPlugin);
    }
}
