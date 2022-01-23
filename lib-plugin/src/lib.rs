#![doc = include_str!("../README.md")]

use bevy::prelude::*;

pub struct HelloPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Person;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Name(String);

struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Name>();
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for Name(name) in query.iter() {
            println!("hello {name}!");
        }
    }
}
