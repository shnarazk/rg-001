#![allow(unused)]

use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use {
    bevy::{asset::LoadState, prelude::*},
    rg_001::{button::ButtonPlugin, state::PlayerPlugin, text::MyTextPlugin},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

fn main() {
    App::new()
        .init_resource::<RpgSpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Setup)
        // from 'state'
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .add_system_set(SystemSet::on_update(AppState::Finished).with_system(animate_sprite_system))
        .add_system_set(
            SystemSet::on_update(AppState::Finished).with_system(print_mouse_events_system),
        )
        // .add_startup_system(setup)
        // .add_system(animate_sprite_system)
        .run()
}

#[derive(Component, Debug, Default)]
struct Player;

#[derive(Component, Debug, Default)]
struct MainCamera;

// (from texture_atlas)

#[derive(Default)]
struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut rpg_sprite_handles: ResMut<RpgSpriteHandles>, asset_server: Res<AssetServer>) {
    rpg_sprite_handles.handles = asset_server.load_folder("dodge/art").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

// from Unofficial Bevy Cheat Book 'Convert cursor to world coodinates'
#[allow(clippy::type_complexity)]
fn print_mouse_events_system(
    // mut mouse_button_input: Res<Input<MouseButton>>,
    windows: ResMut<Windows>,
    mut queries: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<&mut Transform, With<Player>>,
    )>,
) {
    // if mouse_button_input.just_pressed(MouseButton::Left) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = position - size / 2.0;
        let camera_transform = queries.q0().single();
        let clicked = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        let mut q1 = queries.q1();
        let mut trans = &mut q1.single_mut().translation;
        let offset = if 10.0 < (clicked.x - trans.x).abs() && 10.0 < (clicked.y - trans.y).abs() {
            10.0 / 2.0_f32.sqrt()
        } else {
            10.0
        };
        if trans.x + 10.0 < clicked.x {
            trans.x += offset;
        }
        if clicked.x + 10.0 < trans.x {
            trans.x -= offset;
        }
        if trans.y + 10.0 < clicked.y {
            trans.y += offset;
        }
        if clicked.x + 10.0 < trans.x {
            trans.y -= offset;
        }
        // eprintln!(
        //     "Clicked at ({:>.2},{:>.2}) and I'm at ({:>.2},{:>.2})",
        //     clicked.x, clicked.y, trans.x, trans.y,
        // );
    }
}

fn setup(
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
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                // translation: Vec3::new(150.0, 0.0, 0.0),
                // scale: Vec3::splat(4.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.15, true))
        .insert(Player {});
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

// (from 'sprite_sheet')
fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

// Display the player's sprite (from 'sprite')
fn setup1(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("dodge/art/playerGrey_walk1.png"),
        ..Default::default()
    });
}
