use bevy::core::FixedTimestep;
use bevy::prelude::*;
use rand::prelude::random;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

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

#[derive(Component, Debug)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component, Debug)]
struct SnakeSegment;

#[derive(Component, Debug, Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Component, Default, Debug)]
struct Food;

struct Materials {
    head_material: Color,
    food_material: Color,
    segment_material: Color,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_segments(mut commands: Commands, materials: Res<Materials>,
                  position: Position
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: materials.segment_material,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.6))
        .id()
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>,
               mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
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
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(Position { x: 3, y: 4 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segments(
            commands,
            materials,
            Position { x: 3, y: 3 }
        ),
    ];
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut SnakeHead>) {
    for mut head in query.iter_mut() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut query: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>
    )
{
    const W: i32 = ARENA_WIDTH as i32 - 1;
    const H: i32 = ARENA_HEIGHT as i32 - 1;
    for (head_entity, head) in query.iter_mut() {
         let segment_positions = segments.0
             .iter()
             .map(|e| *positions.get_mut(*e).unwrap())
             .collect::<Vec<Position>>();
        let mut pos = positions.get_mut(head_entity).unwrap();
        match head.direction {
            Direction::Left => {
                pos.x = (pos.x - 1).clamp(0, W);
            }
            Direction::Right => {
                pos.x = (pos.x + 1).clamp(0, W);
            }
            Direction::Down => {
                pos.y = (pos.y - 1).clamp(0, H);
            }
            Direction::Up => {
                pos.y = (pos.y + 1).clamp(0, H);
            }
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
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

//
// food
//
fn food_spawner(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: materials.food_material,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<u32>() % ARENA_WIDTH) as i32,
            y: (random::<u32>() % ARENA_HEIGHT) as i32,
        })
        .insert(Size::square(0.8));
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
            food_material: Color::rgb(1.0, 0.2, 0.6),
            segment_material: Color::rgb(0.3, 0.3, 0.3),
        })
        .insert_resource(SnakeSegments::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_snake)
        .add_system(
            snake_movement_input
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(snake_movement.label(SnakeMovement::Movement)),
        )
        .add_system(position_translation)
        .add_system(size_scaling)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(food_spawner),
        )
        .run();
}
