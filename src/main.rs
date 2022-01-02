use {{crate_name}}::*;
use bevy::prelude::*;


fn main() {
    {% if bevy_version == "git" %}App::new(){% else %}App::build(){% endif %}
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
