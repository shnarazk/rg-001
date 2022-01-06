use bevy::prelude::*;
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_simple);
        // .add_system(simple_text_update);
    }
}

#[derive(Component)]
pub struct ScoreLabel;

fn setup_simple(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
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
        .insert(ScoreLabel);
}

fn simple_text_update(time: Res<Time>, mut query: Query<&mut Text, With<ScoreLabel>>) {
    let seconds = time.seconds_since_startup() as f32;
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{}", seconds as u32);
    }
}
