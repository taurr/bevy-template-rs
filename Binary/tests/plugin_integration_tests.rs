use anyhow::Result;
use bevy::prelude::*;
use {{crate_name}}::*;

mod common;

#[test]
fn spawns_entity_with_name() -> Result<()> {
    let mut app = common::bevy_test_app();
    app.add_plugin({{crate_name|pascal_case}}Plugin);

    app.update();

    let e = app
        .world
        .query_filtered::<Entity, With<{{crate_name|pascal_case}}Component>>()
        .iter(&app.world)
        .next()
        .unwrap();

    assert_eq!(app.world.query::<&Name>().get(&app.world, e)?.as_str(), "{{crate_name|pascal_case}}Plugin Root");

    Ok(())
}
