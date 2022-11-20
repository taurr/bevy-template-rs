use {{crate_name}}::*;
use anyhow::Result;

mod common;

#[test]
fn did_hurt_enemy() -> Result {
    let mut app = common::bevy_test_app();
    app.add_plugin({{crate_name|pascal_case}}Plugin);

    // Setup test entities
    let enemy_id = app.world.spawn(Enemy { hit_points: 5 }).id();

    // spin Bevy a few times...
    (0..2).for_each(|_| app.update());

    // Check resulting changes
    assert_eq!(
        app.world
            .get::<Enemy>(enemy_id)
            .map(|enemy| enemy.hit_points),
        Some(3)
    );

    Ok(())
}
