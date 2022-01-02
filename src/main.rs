use {
    bevy::prelude::*,
    rg_001::{button::ButtonPlugin, greet::GreetPlugin, state::PlayerPlugin, text::MyTextPlugin},
};

fn main() {
    // Defaultplugins is CorePlugin, InputPlugin and WindowPlugin.
    App::new()
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 1.0 / 5.0f32})
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(GreetPlugin)
        .add_plugin(MyTextPlugin)
        .add_plugin(ButtonPlugin)
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .run();
}

fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<StandardMaterial>>,
         asset_server: Res<AssetServer>)
{
    let society_mesh = asset_server.load("models/society.glb#Mesh0/Primitive0");
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.8, 0.9),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: society_mesh,
        material: material_handle,
        transform: Transform::from_xyz(-3.0, 0.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
} 

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}
