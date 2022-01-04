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

struct GrowthEvent;

#[derive(Default)]
struct LastTailPosition(Option<Position>);

struct GameOverEvent;

//
// functions
//
fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_segment(mut commands: Commands, materials: Res<Materials>, position: Position) -> Entity {
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

fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
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
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 4 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, materials, Position { x: 3, y: 3 }),
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
    mut last_tail_position: ResMut<LastTailPosition>,
    mut query: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    for (head_entity, head) in query.iter_mut() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut pos = positions.get_mut(head_entity).unwrap();
        match head.direction {
            Direction::Left => {
                pos.x -= 1;
            }
            Direction::Right => {
                pos.x += 1;
            }
            Direction::Down => {
                pos.y -= 1;
            }
            Direction::Up => {
                pos.y += 1;
            }
        }
        if pos.x < 0
            || pos.y < 0
            || pos.x as u32 >= ARENA_WIDTH
            || pos.y as u32 >= ARENA_HEIGHT
            || segment_positions.contains(&pos)
        {
            game_over_writer.send(GameOverEvent);
            pos.x = (random::<u32>() % ARENA_WIDTH) as i32;
            pos.y = (random::<u32>() % ARENA_HEIGHT) as i32;
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        last_tail_position.0 = Some(*segment_positions.last().unwrap());
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

fn snake_grow(
    commands: Commands,
    last_tail_position: ResMut<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<Materials>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            materials,
            last_tail_position.0.unwrap(),
        ))
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

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, _) in food_positions.iter().filter(|(_, f)| *f == head_pos) {
            commands.entity(ent).despawn();
            growth_writer.send(GrowthEvent)
        }
    }
}

// game over
fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<Materials>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in segments.iter().chain(food.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, materials, segments_res);
    }
}

//
// Main
//
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
            segment_material: Color::rgb(0.4, 0.4, 0.4),
        })
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
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
                .with_system(snake_movement.label(SnakeMovement::Movement))
                .with_system(
                    snake_eating
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement),
                )
                .with_system(
                    snake_grow
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating),
                ),
        )
        .add_system(position_translation)
        .add_system(size_scaling)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(food_spawner),
        )
        .add_system(game_over.after(SnakeMovement::Movement))
        .run();
}
