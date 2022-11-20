use bevy::prelude::*;

pub(crate) struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    #[cfg(not(debug_assertions))]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
    }

    #[cfg(debug_assertions)]
    #[cfg(not(any(feature = "editor", feature = "inspector")))]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            //.add(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            //.add(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Image>::default())
            .add(bevy::diagnostic::LogDiagnosticsPlugin::default())
    }

    #[cfg(debug_assertions)]
    #[cfg(all(feature = "editor", not(feature = "inspector")))]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add(bevy_editor_pls::EditorPlugin)
    }

    #[cfg(debug_assertions)]
    #[cfg(all(not(feature = "editor"), feature = "inspector"))]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        use bevy_inspector_egui::WorldInspectorPlugin;
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add(WorldInspectorPlugin::new())
    }
}
