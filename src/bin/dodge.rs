// #![allow(unused)]
use bevy::{
    asset::LoadState, core::FixedTimestep, input::system::exit_on_esc_system, prelude::*,
    sprite::collide_aabb::collide,
};
use rand::prelude::random;
use rg_001::dodge_text::{ScoreLabel, ScorePlugin};

const Z_AXIS: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Ready,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dodge!".to_string(),
            width: 1200.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.6, 0.8, 1.0)))
        .init_resource::<GameResourceHandles>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ScorePlugin)
        .add_state(AppState::Setup)
        // from 'state'
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_assets))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_cammera))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_player))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(play_bgm))
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_run_criteria(FixedTimestep::step(25.5))
                .with_system(play_bgm),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_run_criteria(FixedTimestep::step(0.55))
                .with_system(setup_enemy),
        )
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_player))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_enemy))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(check_collision))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(track_mouse_movement))
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(update_score),
        )
        .add_system(exit_on_esc_system)
        .run()
}

//
// Character, autonomous moving objects
//
#[derive(Component, Debug)]
struct Character {
    texture_atlas: TextureAtlas,
    flip: bool,
    diff_x: f32,
    diff_y: f32,
    trans_x: f32,
    trans_y: f32,
}

impl Character {
    fn from(texture_atlas: TextureAtlas) -> Self {
        Self {
            texture_atlas,
            flip: false,
            diff_x: 0.0,
            diff_y: 0.0,
            trans_x: 0.0,
            trans_y: 0.0,
        }
    }
    fn with_direction(mut self, x: f32, y: f32) -> Self {
        self.diff_x = x;
        self.diff_y = y;
        self
    }
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity, dead_code)]
fn animate_character(
    time: Res<Time>,
    mut query: Query<(
        &mut Character,
        &mut Timer,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut character, mut timer, mut trans, mut sprite) in query.iter_mut() {
        trans.translation.x += character.diff_x;
        trans.translation.y += character.diff_y;
        // memoize the location after moving
        character.trans_x = trans.translation.x;
        character.trans_y = trans.translation.y;
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % character.texture_atlas.textures.len();
            sprite.flip_x = character.flip;
        }
    }
}

//
// Player
//
#[derive(Component, Debug, Default)]
struct Player {
    score: f32,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in [
        asset_server.get_handle("sprites/bevy_logo_dark_1.png"),
        asset_server.get_handle("sprites/bevy_logo_dark_2.png"),
        asset_server.get_handle("sprites/bevy_logo_dark_3.png"),
    ] {
        if let Some(image) = textures.get(&handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = asset_server.load("sprites/bevy_logo_dark_1.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, Z_AXIS),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.15, true))
        .insert(Character::from(texture_atlas))
        .insert(Player::default());
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
fn animate_player(
    config: Res<WindowDescriptor>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Character,
            &mut Timer,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    for (mut player, mut timer, mut trans, mut sprite) in query.iter_mut() {
        trans.translation.x =
            (trans.translation.x + player.diff_x).clamp(-0.45 * config.width, 0.45 * config.width);
        trans.translation.y = (trans.translation.y + player.diff_y)
            .clamp(-0.45 * config.height, 0.45 * config.height);
        player.trans_x = trans.translation.x;
        player.trans_y = trans.translation.y;
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % player.texture_atlas.textures.len();
            sprite.flip_x = player.flip;
        }
    }
}

//
// Enemy
//
#[derive(Debug)]
enum EnemyKind {
    Fly,
    Swim,
    Walk,
}

#[derive(Component, Debug)]
struct Enemy {
    kind: EnemyKind,
    collided: bool,
}

fn setup_enemy(
    mut commands: Commands,
    config: Res<WindowDescriptor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let (kind, sprites) = match (random::<f32>() * 3.0) as usize {
        1 => (
            EnemyKind::Swim,
            [
                asset_server.get_handle("dodge/art/enemySwimming_1.png"),
                asset_server.get_handle("dodge/art/enemySwimming_2.png"),
            ],
        ),
        2 => (
            EnemyKind::Walk,
            [
                asset_server.get_handle("dodge/art/enemyWalking_1.png"),
                asset_server.get_handle("dodge/art/enemyWalking_2.png"),
            ],
        ),
        _ => (
            EnemyKind::Fly,
            [
                asset_server.get_handle("dodge/art/enemyFlyingAlt_1.png"),
                asset_server.get_handle("dodge/art/enemyFlyingAlt_2.png"),
            ],
        ),
    };
    for handle in sprites {
        if let Some(image) = textures.get(&handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = match kind {
        EnemyKind::Fly => asset_server.load("dodge/art/enemyFlyingAlt_1.png"),
        EnemyKind::Swim => asset_server.load("dodge/art/enemySwimming_1.png"),
        EnemyKind::Walk => asset_server.load("dodge/art/enemyWalking_1.png"),
    };
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    let mut px = 0.5 * random::<f32>() * config.width;
    let mut py = 0.5 * random::<f32>() * config.height;
    let mut dx;
    let mut dy;
    match (random::<f32>() * 4.0) as usize {
        1 => {
            px = config.width * 0.5 - 40.0;
            dx = -1.0;
            dy = random::<f32>() - 0.5;
        }
        2 => {
            px = -(config.width * 0.5 - 40.0);
            dx = 1.0;
            dy = random::<f32>() - 0.5;
        }
        3 => {
            py = config.height * 0.5 - 40.0;
            dx = random::<f32>() - 0.5;
            dy = -1.0;
        }
        _ => {
            py = -(config.height * 0.5 - 40.0);
            dx = random::<f32>() - 0.5;
            dy = 1.0;
        }
    }
    const SPEED: f32 = 7.5;
    let dist: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
    assert!(dist < 2.0);
    dx *= SPEED / dist;
    dy *= SPEED / dist;
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(px, py, Z_AXIS),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.15, true))
        .insert(Character::from(texture_atlas).with_direction(dx, dy))
        .insert(Enemy {
            kind,
            collided: false,
        });
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
fn animate_enemy(
    // mut commands: Commands,
    config: Res<WindowDescriptor>,
    time: Res<Time>,
    mut query: Query<(
        // Entity,
        &mut Character,
        &mut Timer,
        &mut Transform,
        &mut TextureAtlasSprite,
        &mut Enemy,
    )>,
) {
    for (mut enemy, mut timer, mut trans, mut sprite, mut et) in query.iter_mut() {
        trans.translation.x += enemy.diff_x;
        trans.translation.y += enemy.diff_y;
        trans.rotation = Quat::from_rotation_z(enemy.diff_y.atan2(enemy.diff_x));
        enemy.trans_x = trans.translation.x;
        enemy.trans_y = trans.translation.y;
        enemy.diff_x *= 1.01;
        enemy.diff_y *= 1.01;
        if 0.5 * config.width < enemy.trans_x.abs() && 0.5 * config.height < enemy.trans_y.abs() {
            // commands.entity(ent).despawn();

            let mut px = 0.5 * random::<f32>() * config.width;
            let mut py = 0.5 * random::<f32>() * config.height;
            let mut dx;
            let mut dy;
            match (random::<f32>() * 4.0) as usize {
                1 => {
                    px = config.width * 0.5 - 40.0;
                    dx = -1.0;
                    dy = random::<f32>() - 0.5;
                }
                2 => {
                    px = -(config.width * 0.5 - 40.0);
                    dx = 1.0;
                    dy = random::<f32>() - 0.5;
                }
                3 => {
                    py = config.height * 0.5 - 40.0;
                    dx = random::<f32>() - 0.5;
                    dy = -1.0;
                }
                _ => {
                    py = -(config.height * 0.5 - 40.0);
                    dx = random::<f32>() - 0.5;
                    dy = 1.0;
                }
            }
            let speed: f32 = match et.kind {
                EnemyKind::Fly => 9.0,
                EnemyKind::Swim => 6.2,
                EnemyKind::Walk => 4.0,
            };
            let dist: f32 = (dx.powi(2) + dy.powi(2)).sqrt();
            dx *= speed / dist;
            dy *= speed / dist;

            trans.translation.x = px;
            trans.translation.y = py;
            enemy.trans_x = px;
            enemy.trans_y = py;
            enemy.diff_x = dx;
            enemy.diff_y = dy;
            et.collided = false;
        }
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % enemy.texture_atlas.textures.len();
        }
    }
}

//
// Collision detection
//
fn check_collision(
    mut player_query: Query<(&Transform, &mut Player)>,
    collider_query: Query<(&Transform, &Enemy)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let (player_trans, mut player) = player_query.single_mut();
    // let player_size = player_trans.scale.truncate();
    for (enemy_trans, enemy) in collider_query.iter() {
        if enemy.collided {
            continue;
        }
        if let Some(_collision) = collide(
            player_trans.translation,
            Vec2::new(40.0, 40.0), // player_size,
            enemy_trans.translation,
            Vec2::new(40.0, 40.0), // enemy_trans.scale.truncate(),
        ) {
            player.score *= 0.5;
            if player.score < 1.0 {
                // should be game over by shifting to the next stage
            } else {
                audio.play(asset_server.get_handle("sound/forceField_000.ogg"));
            }
        }
    }
}

//
// Configuration
//
// (from texture_atlas)
#[derive(Default)]
struct GameResourceHandles {
    sprites: Vec<HandleUntyped>,
    sounds: Vec<HandleUntyped>,
}

// impl FromWorld for CharacterSpriteHandles {
//     fn from_world(world: &mut World) -> Self {
//         let world = world.cell();
//         let asset_server = world.get_resource::<AssetServer>().unwrap();
//         asset_server.watch_for_changes().unwrap();
//         let mut handles = asset_server.load_folder("dodge/art").unwrap();
//         handles.append(&mut asset_server.load_folder("sprites").unwrap());
//         Self { handles }
//     }
// }

fn load_assets(mut handles: ResMut<GameResourceHandles>, asset_server: ResMut<AssetServer>) {
    handles.sprites = asset_server.load_folder("dodge/art").unwrap();
    handles
        .sprites
        .append(&mut asset_server.load_folder("sprites").unwrap());
    handles.sounds = asset_server.load_folder("sound").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<GameResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.sprites.iter().map(|handle| handle.id))
    {
        state.set(AppState::Ready).unwrap();
    }
}

// from Unofficial Bevy Cheat Book 'Convert cursor to world coodinates'
#[allow(clippy::type_complexity)]
fn track_mouse_movement(
    windows: ResMut<Windows>,
    mut queries: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<&mut Character, With<Player>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = position - size / 2.0;
        let camera_transform = queries.q0().single();
        let clicked = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let mut q1 = queries.q1();
        let mut player = q1.single_mut();
        let dx = clicked.x - player.trans_x;
        let dy = clicked.y - player.trans_y;
        let dist2 = dx.powi(2) + dy.powi(2);
        if 100.0 < dist2 {
            let dist = dist2.sqrt();
            player.flip = dx < 0.0;
            player.diff_x = 10.0 * dx / dist;
            player.diff_y = 10.0 * dy / dist;
        } else {
            player.flip = false;
            player.diff_x = 0.0;
            player.diff_y = 0.0;
        }
    }
}

fn update_score(
    mut player_query: Query<&mut Player>,
    mut score_query: Query<&mut Text, With<ScoreLabel>>,
) {
    let mut player = player_query.single_mut();
    player.score += 1.0;
    let mut score = score_query.single_mut();
    score.sections[1].value = format!("{}", player.score as u32);
}

//
// Camera
//
#[derive(Component, Debug, Default)]
struct MainCamera;

fn setup_cammera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

//
// BGM
//
#[allow(dead_code)]
fn play_bgm(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // let music = asset_server.get_handle("dodge/art/House In a Forest Loop.ogg");
    let music = asset_server.get_handle("sound/Windless Slopes.ogg");
    audio.play(music);
}
