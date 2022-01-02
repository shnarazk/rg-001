use {
    bevy::prelude::*,
    rg_001::{button::ButtonPlugin, state::PlayerPlugin, text::MyTextPlugin},
};

fn main() {
    // Defaultplugins is CorePlugin, InputPlugin and WindowPlugin.
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        // .add_plugin(GreetPlugin)
        .add_plugin(MyTextPlugin)
        .add_plugin(ButtonPlugin)
        .run();
}
