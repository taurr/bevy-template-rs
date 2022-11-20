use bevy::{prelude::*, render::settings::WgpuSettings, winit::WinitPlugin};

pub(crate) fn bevy_test_app() -> App {
    let mut app = App::new();

    app.insert_resource(WgpuSettings {
        backends: None,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>());

    app
}
