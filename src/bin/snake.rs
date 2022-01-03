use bevy::prelude::*;

#[derive(Component, Default, Debug)]
struct SnakeHead;

struct Materials {
    head_material: Color,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.8, 0.7)))
        .insert_resource(Materials {
            head_material: Color::rgb(0.7, 0.7, 0.7),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_snake)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: materials.head_material,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}
