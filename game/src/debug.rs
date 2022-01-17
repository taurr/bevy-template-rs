use bevy::prelude::*;
#[cfg(feature = "egui_inspector")]
use bevy_inspector_egui::{
    plugin::InspectorWindows, Inspectable, InspectorPlugin, WorldInspectorPlugin,
};

pub trait BevyDebug {
    #[cfg(feature = "egui_inspector")]
    fn add_world_inspector(&mut self) -> &mut Self;

    #[cfg(feature = "egui_inspector")]
    fn add_inspector<T>(&mut self) -> &mut Self
    where
        T: Send + Sync + FromWorld + Inspectable + 'static;

    #[cfg(feature = "write_graphs")]
    fn write_graphs(&mut self) -> std::io::Result<&mut Self>;
}

#[cfg(feature = "egui_inspector")]
#[allow(unused)]
fn toggle_inspector<T: Inspectable + 'static>(
    input: ResMut<Input<KeyCode>>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    use bevy_inspector_egui::plugin::InspectorWindows;

    if input.just_pressed(KeyCode::F12) {
        let mut inspector_window_data = inspector_windows.window_data_mut::<T>();
        inspector_window_data.visible = !inspector_window_data.visible;
    }
}

impl BevyDebug for App {
    #[cfg(feature = "egui_inspector")]
    fn add_world_inspector(&mut self) -> &mut Self {
        self.add_plugin(WorldInspectorPlugin::new());
        self
    }

    #[cfg(feature = "egui_inspector")]
    fn add_inspector<T>(&mut self) -> &mut Self
    where
        T: Send + Sync + FromWorld + Inspectable + 'static,
    {
        self.add_plugin(InspectorPlugin::<T>::new());
        self.add_system(toggle_inspector::<T>);
        self
    }

    #[cfg(feature = "write_graphs")]
    fn write_graphs(&mut self) -> std::io::Result<&mut Self> {
        graphs::write_schedule_graph(self, "schedule.dot")?;
        graphs::write_render_graph(self, "render.dot")?;
        graphs::write_render_schedule_graph(self, "render_schedule.dot")?;
        Ok(self)
    }
}

#[cfg(feature = "write_graphs")]
mod graphs {
    use bevy::prelude::App;

    pub(super) fn write_schedule_graph(app: &App, filename: &str) -> std::io::Result<()> {
        let dot = { bevy_mod_debugdump::schedule_graph::schedule_graph_dot(app) };
        let mut file = std::fs::File::create(filename)?;
        std::io::Write::write_all(&mut file, dot.as_bytes())?;
        Ok(())
    }

    pub(super) fn write_render_graph(app: &App, filename: &str) -> std::io::Result<()> {
        let dot = {
            use bevy::render::render_graph::RenderGraph;
            let render_app = app
                .get_sub_app(bevy::render::RenderApp)
                .expect("no render app");
            let render_graph = render_app.world.get_resource::<RenderGraph>().unwrap();

            bevy_mod_debugdump::render_graph::render_graph_dot(&*render_graph)
        };
        let mut file = std::fs::File::create(filename)?;
        std::io::Write::write_all(&mut file, dot.as_bytes())?;
        Ok(())
    }

    pub(super) fn write_render_schedule_graph(
        app: &mut App,
        filename: &str,
    ) -> std::io::Result<()> {
        let dot = {
            app.update();

            let default_style = bevy_mod_debugdump::schedule_graph::ScheduleGraphStyle {
                hide_startup_schedule: false,
                ..bevy_mod_debugdump::schedule_graph::ScheduleGraphStyle::dark()
            };
            bevy_mod_debugdump::schedule_graph::schedule_graph_dot_sub_app_styled(
                app,
                bevy::render::RenderApp,
                &[&bevy::render::RenderStage::Extract],
                &default_style,
            )
        };
        let mut file = std::fs::File::create(filename)?;
        std::io::Write::write_all(&mut file, dot.as_bytes())?;
        Ok(())
    }
}
