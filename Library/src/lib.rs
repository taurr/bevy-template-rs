#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub struct {{crate_name|pascal_case}}Plugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Person;

#[derive(Deref, DerefMut)]
struct GreetTimer(Timer);

impl Plugin for {{crate_name|pascal_case}}Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(Self::add_people)
            .add_system(Self::greet_people);
    }
}

impl {{crate_name|pascal_case}}Plugin {
    fn add_people(mut commands: Commands) {
        for &name in ["Elaina Proctor", "Renzo Hume", "Zayna Nieves"].iter() {
            commands.spawn().insert(Person).insert(Name::new(name));
        }
    }

    fn greet_people(
        time: Res<Time>,
        mut timer: ResMut<GreetTimer>,
        query: Query<&Name, With<Person>>,
    ) {
        if timer.tick(time.delta()).just_finished() {
            for name in query.iter() {
                println!("hello {name}!");
            }
        }
    }
}
