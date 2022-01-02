use bevy::prelude::*;

use crate::state::{PersonState, Person};

pub struct MyTextPlugin;

impl Plugin for MyTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_simple)
            .add_startup_system(setup_color)
            .add_system(simple_text_update)
            .add_system(color_text_update);
    }
}

#[derive(Component)]
pub struct SimpleText;

fn setup_simple(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "名前: ".to_string(),
                        style: TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font: asset_server.load("fonts/NotoSansJP-Regular.otf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            // font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font: asset_server.load("fonts/NotoSansJP-Regular.otf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SimpleText);
}

fn simple_text_update(time: Res<Time>, mut query: Query<&mut Text, With<SimpleText>>, state: Query<&PersonState, With<Person>>) {
    let _seconds = time.seconds_since_startup() as f32;
    for person in state.iter() {
        if person.name == "Me" {
            for mut text in query.iter_mut() {
                // text.sections[1].value = format!("{:>.2}", seconds);
                text.sections[1].value = person.name.clone();
            }
            break;
        }
    }
}

#[derive(Component)]
struct ColorText;

fn setup_color(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ColorText);
}

fn color_text_update(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
