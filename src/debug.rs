use bevy::prelude::*;

pub trait BevyDebug {
    fn add_inspector(&mut self) -> &mut Self;
    fn write_graphs(&mut self) -> std::io::Result<&mut Self>;
}

impl BevyDebug for App {
    fn add_inspector(&mut self) -> &mut Self {
        #[cfg(feature = "egui_inspector")]
        self.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
        self
    }

    #[cfg(not(feature = "write_graphs"))]
    fn write_graphs(&mut self) -> std::io::Result<&mut Self> {
        Ok(self)
    }

    #[cfg(feature = "write_graphs")]
    fn write_graphs(&mut self) -> std::io::Result<&mut Self> {
        write_schedule_graph(self, "schedule.dot")?;
        write_render_graph(self, "render.dot")?;
        write_render_schedule_graph(self, "render_schedule.dot")?;
        Ok(self)
    }
}

#[cfg(feature = "write_graphs")]
fn write_schedule_graph(app: &App, filename: &str) -> std::io::Result<()> {
    let dot = { bevy_mod_debugdump::schedule_graph::schedule_graph_dot(app) };
    let mut file = std::fs::File::create(filename)?;
    std::io::Write::write_all(&mut file, dot.as_bytes())?;
    Ok(())
}

#[cfg(feature = "write_graphs")]
fn write_render_graph(app: &App, filename: &str) -> std::io::Result<()> {
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

#[cfg(feature = "write_graphs")]
fn write_render_schedule_graph(app: &mut App, filename: &str) -> std::io::Result<()> {
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
