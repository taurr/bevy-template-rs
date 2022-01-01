use bevy::prelude::*;

fn main() {
    {% if bevy_version == "git" %}App::new(){%else%}App::build(){%endif%}
        .add_system(hello_world.system())
        .run();
}

fn hello_world() {
    println!("hello world!");
}
