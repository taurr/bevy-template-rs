use bevy::prelude::*;

fn main() {
    {% if bevy_version == "git" %}App::new(){%else%}App::build(){%endif%}
        .add_system(hello_world{% if bevy_version != "git" %}.system(){%endif%})
        .run();
}

fn hello_world() {
    println!("hello world!");
}
