use bevy::prelude::*;

pub struct HelloPlugin;

fn hello_world() {
    println!("Hello, world!");
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people);
            // .add_system(hello_world);
    }
}

#[derive(Component, Default)]
struct Person;

#[derive(Component, Default)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Me".to_string()));
    commands.spawn().insert(Person).insert(Name("Alice".to_string()));
}

pub struct GreetPlugin;
struct GreetTimer(Timer);

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_system(greet_people);
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}!", name.0);
        }
    }
}
