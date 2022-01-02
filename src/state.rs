use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Person;

#[derive(Component, Debug, Default)]
pub struct PersonState {
    pub name: String,
    pub x: usize,
    pub y: usize,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(PersonState {
        name: "Me".to_string(),
        x: 3,
        y: 0,
    });
    commands.spawn().insert(Person).insert(PersonState {
        name: "Alice".to_string(),
        x: 3,
        y: 0,
    });
}
