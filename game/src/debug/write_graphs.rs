use bevy::prelude::*;
use bevy::render::{render_graph::RenderGraph, RenderApp, RenderStage};
use bevy_mod_debugdump::{
    render_graph::render_graph_dot,
    schedule_graph::{schedule_graph_dot, schedule_graph_dot_sub_app_styled, ScheduleGraphStyle},
};
use std::fs::File;
use std::io::{Result, Write};

pub trait WriteGraphs {
    fn write_graphs(&mut self) -> Result<&mut Self>;
}

impl WriteGraphs for App {
    fn write_graphs(&mut self) -> Result<&mut Self> {
        write_schedule_graph(self, "schedule.dot")?;
        write_render_graph(self, "render.dot")?;
        write_render_schedule_graph(self, "render_schedule.dot")?;
        Ok(self)
    }
}

fn write_schedule_graph(app: &App, filename: &str) -> Result<()> {
    let dot = schedule_graph_dot(app);
    let mut file = File::create(filename)?;
    Write::write_all(&mut file, dot.as_bytes())?;
    Ok(())
}

fn write_render_graph(app: &App, filename: &str) -> Result<()> {
    let dot = {
        let render_app = app.get_sub_app(RenderApp).expect("no render app");
        let render_graph = render_app.world.get_resource::<RenderGraph>().unwrap();

        render_graph_dot(&*render_graph)
    };
    let mut file = File::create(filename)?;
    Write::write_all(&mut file, dot.as_bytes())?;
    Ok(())
}

fn write_render_schedule_graph(app: &mut App, filename: &str) -> Result<()> {
    let dot = {
        app.update();

        let default_style = ScheduleGraphStyle {
            hide_startup_schedule: false,
            ..ScheduleGraphStyle::dark()
        };
        schedule_graph_dot_sub_app_styled(app, RenderApp, &[&RenderStage::Extract], &default_style)
    };
    let mut file = File::create(filename)?;
    Write::write_all(&mut file, dot.as_bytes())?;
    Ok(())
}
