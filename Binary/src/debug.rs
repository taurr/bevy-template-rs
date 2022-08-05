use bevy::prelude::*;

pub(crate) struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    #[cfg(not(debug_assertions))]
    fn build(&mut self, _: &mut bevy::app::PluginGroupBuilder) {}

    #[cfg(debug_assertions)]
    fn build(&mut self, #[allow(unused)] group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add(bevy::diagnostic::EntityCountDiagnosticsPlugin);

        #[cfg(not(feature = "editor"))]
        group.add(bevy::diagnostic::LogDiagnosticsPlugin::default());

        #[cfg(feature = "editor")]
        group.add(bevy_editor_pls::EditorPlugin);
    }
}
