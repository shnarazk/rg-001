use {
    crate::state::{Person, PersonState},
    bevy::prelude::*,
};

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(10.0, true)))
            .add_system(greet_people);
    }
}

struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&PersonState, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for state in query.iter() {
            println!("Hello {}!", state.name);
        }
    }
}

#[allow(dead_code)]
fn hello_world() {
    println!("Hello, world!");
}
