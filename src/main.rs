use {
    bevy::prelude::*,
    rg_001::{greet::GreetPlugin, text::MyTextPlugin},
};

fn main() {
    println!("Hello, world!");
    // Defaultplugins is CorePlugin, InputPlugin and WindowPlugin.
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GreetPlugin)
        .add_plugin(MyTextPlugin)
        .run();
}
