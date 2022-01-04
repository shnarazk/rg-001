#![allow(unused)]
use bevy::{asset::LoadState, input::system::exit_on_esc_system, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Ready,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dodge!".to_string(),
            width: 1400.0,
            height: 800.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_resource::<CharacterSpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Setup)
        // from 'state'
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_cammera))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_player))
        .add_system_set(SystemSet::on_enter(AppState::Ready).with_system(setup_enemy))
        // .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_player))
        // .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_enemy))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(animate_character))
        .add_system_set(SystemSet::on_update(AppState::Ready).with_system(track_mouse_movement))
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
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
fn animate_character(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
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
struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let texture0: Handle<Image> = asset_server.get_handle("dodge/art/playerGrey_walk1.png");
    let texture1: Handle<Image> = asset_server.get_handle("dodge/art/playerGrey_walk2.png");
    for handle in [texture0, texture1] {
        if let Some(image) = textures.get(&handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = asset_server.load("dodge/art/playerGrey_walk1.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.15, true))
        .insert(Character::from(texture_atlas))
        .insert(Player);
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
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
        trans.translation.x += player.diff_x;
        trans.translation.y += player.diff_y;
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
#[derive(Component, Debug, Default)]
struct Enemy;

fn setup_enemy(
    mut commands: Commands,
    // config: Res<WindowDescriptor>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in [
        asset_server.get_handle("dodge/art/enemyFlyingAlt_1.png"),
        asset_server.get_handle("dodge/art/enemyFlyingAlt_2.png"),
        asset_server.get_handle("dodge/art/enemySwimming_1.png"),
        asset_server.get_handle("dodge/art/enemySwimming_2.png"),
        asset_server.get_handle("dodge/art/enemyWalking_1.png"),
        asset_server.get_handle("dodge/art/enemyWalking_2.png"),
    ] {
        if let Some(image) = textures.get(&handle) {
            texture_atlas_builder.add_texture(handle.clone_weak(), image);
        }
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = asset_server.load("dodge/art/enemyWalking_1.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas.clone());

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(-100.0, 0.0, 0.0),
                scale: Vec3::splat(0.5),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.15, true))
        .insert(Character::from(texture_atlas))
        .insert(Enemy);
}

// (from 'sprite_sheet')
#[allow(clippy::type_complexity)]
fn animate_enemy(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut Character,
            &mut Timer,
            &mut Transform,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
) {
    for (mut enemy, mut timer, mut trans, mut sprite) in query.iter_mut() {
        trans.translation.x += enemy.diff_x;
        trans.translation.y += enemy.diff_y;
        enemy.trans_x = trans.translation.x;
        enemy.trans_y = trans.translation.y;
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % enemy.texture_atlas.textures.len();
            sprite.flip_x = enemy.flip;
        }
    }
}

//
// Configuration
//
// (from texture_atlas)
#[derive(Default)]
struct CharacterSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(
    mut rpg_sprite_handles: ResMut<CharacterSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    rpg_sprite_handles.handles = asset_server.load_folder("dodge/art").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<CharacterSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
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
