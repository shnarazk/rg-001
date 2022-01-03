use bevy::prelude::*;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Default, Copy, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Default, Copy, Clone)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component, Default, Debug)]
struct SnakeHead;

struct Materials {
    head_material: Color,
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
        .insert(Timer::from_seconds(0.05, true))
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 4 })
        .insert(Size::square(1.0));
}

fn snake_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<(&mut Timer, &mut Position), With<SnakeHead>>,
) {
    const W: i32 = ARENA_WIDTH as i32 - 1;
    const H: i32 = ARENA_HEIGHT as i32 - 1;
    for (mut timer, mut pos) in head_positions.iter_mut() {
        timer.tick(time.delta());
        if !timer.finished() {
            continue;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x = (pos.x - 1).clamp(0, W);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x = (pos.x + 1).clamp(0, W);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y = (pos.y - 1).clamp(0, H);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y = (pos.y + 1).clamp(0, H);
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.custom_size = Some(Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        ));
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
        .insert_resource(Materials {
            head_material: Color::rgb(0.7, 0.7, 0.7),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system(position_translation)
        .add_system(size_scaling)
        .run();
}
