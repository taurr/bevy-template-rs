use bevy::prelude::PluginGroup;

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    #[cfg(feature = "editor")]
    fn build(&mut self, #[allow(unused)] group: &mut bevy::app::PluginGroupBuilder) {
        use bevy_editor_pls::EditorPlugin;
        group
            .add(EditorPlugin)
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add(bevy::diagnostic::EntityCountDiagnosticsPlugin);
    }

    #[cfg(not(feature = "editor"))]
    fn build(&mut self, _: &mut bevy::app::PluginGroupBuilder) {
    }
}
