use bevy::prelude::*;
use bevy_inspector_egui::{
    plugin::InspectorWindows, Inspectable, InspectorPlugin, WorldInspectorPlugin,
};

pub use bevy::ecs::query::{FilterFetch, WorldQuery};
pub use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};

#[derive(Debug)]
pub struct InspectorParams {
    pub key: KeyCode,
}

pub trait Inspector {
    fn add_world_inspector(&mut self) -> &mut Self;

    fn add_filtered_world_inspector<F>(&mut self) -> &mut Self
    where
        F: WorldQuery + 'static,
        F::Fetch: FilterFetch;

    fn add_inspector<T>(&mut self) -> &mut Self
    where
        T: Send + Sync + FromWorld + Inspectable + 'static;
}

impl Default for InspectorParams {
    fn default() -> Self {
        Self { key: KeyCode::F9 }
    }
}

impl Inspector for App {
    fn add_world_inspector(&mut self) -> &mut Self {
        let world = &mut self.world;
        world
            .get_resource_or_insert_with(|| WorldInspectorParams {
                enabled: false,
                ..Default::default()
            })
            .get_resource_or_insert_with(InspectorParams::default);

        self.add_plugin(WorldInspectorPlugin::new());
        self.add_system(toggle_world_inspector);
        self
    }

    fn add_filtered_world_inspector<F>(&mut self) -> &mut Self
    where
        F: WorldQuery + 'static,
        F::Fetch: FilterFetch,
    {
        let world = &mut self.world;
        world
            .get_resource_or_insert_with(|| WorldInspectorParams {
                enabled: false,
                ..Default::default()
            })
            .get_resource_or_insert_with(InspectorParams::default);

        self.add_plugin(WorldInspectorPlugin::new().filter::<F>());
        self.add_system(toggle_world_inspector);
        self
    }

    fn add_inspector<T>(&mut self) -> &mut Self
    where
        T: Send + Sync + FromWorld + Inspectable + 'static,
    {
        let world = &mut self.world;
        world
            .get_resource_or_insert_with(|| WorldInspectorParams {
                enabled: false,
                ..Default::default()
            })
            .get_resource_or_insert_with(InspectorParams::default);

        self.add_plugin(InspectorPlugin::<T>::new());
        self.add_system(toggle_inspector::<T>);
        self
    }
}

fn toggle_inspector<T: Inspectable + 'static>(
    input: ResMut<Input<KeyCode>>,
    inspector_params: Res<InspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    if input.just_pressed(inspector_params.key) {
        let mut inspector_window_data = inspector_windows.window_data_mut::<T>();
        inspector_window_data.visible = !inspector_window_data.visible;
    }
}

fn toggle_world_inspector(
    input: ResMut<Input<KeyCode>>,
    inspector_params: Res<InspectorParams>,
    mut world_params: ResMut<bevy_inspector_egui::WorldInspectorParams>,
) {
    if input.just_pressed(inspector_params.key) {
        world_params.enabled = !world_params.enabled;
    }
}
