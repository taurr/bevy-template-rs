use bevy::prelude::*;

pub struct HelloPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Person;

#[derive(Deref, DerefMut)]
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
        .insert(Name::new("Elaina Proctor"));
    commands
        .spawn()
        .insert(Person)
        .insert(Name::new("Renzo Hume"));
    commands
        .spawn()
        .insert(Person)
        .insert(Name::new("Zayna Nieves"));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {name}!");
        }
    }
}
