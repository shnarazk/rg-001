use {
    bevy::prelude::*,
    rg_001::{button::ButtonPlugin, greet::GreetPlugin, state::PlayerPlugin, text::MyTextPlugin},
};

fn main() {
    // Defaultplugins is CorePlugin, InputPlugin and WindowPlugin.
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 3.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(GreetPlugin)
        .add_plugin(MyTextPlugin)
        .add_plugin(ButtonPlugin)
        .add_startup_system(setup)
        .add_system(animate_light_direction)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const D_STEP: f32 = std::f32::consts::PI / 10.0;
    const PI_2: f32 = std::f32::consts::FRAC_PI_2;
    const R: f32 = 2.0;
    let scale: f32 = 0.5;
    {
        let mesh = asset_server.load("models/logo.glb#Mesh0/Primitive0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 1.0, 0.8),
            ..Default::default()
        });
        let transform = Transform::from_xyz(0.0, 2.4, -4.0)
            .with_rotation(Quat::from_rotation_x(PI_2))
            .with_scale(Vec3::new(0.8, 0.8, 0.8));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }
    // pawns
    {
        let mesh = asset_server.load("models/char-dollar.glb#Mesh0/Primitive0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.7, 0.3, 0.9),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(
                R * (0.0 * D_STEP).cos(),
                R * (0.0 * D_STEP).sin(),
                0.0,
            ))
            .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }
    {
        let mesh = asset_server.load("models/society.glb#Mesh0/Primitive0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.9, 0.8, 0.2),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(
                R * (1.0 * D_STEP).cos(),
                R * (1.0 * D_STEP).sin(),
                0.0,
            ))
            .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }
    {
        let mesh = asset_server.load("models/char-U.glb#Mesh0/Primitive0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.8, 0.9),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(
                R * (2.0 * D_STEP).cos(),
                R * (2.0 * D_STEP).sin(),
                0.0,
            ))
            .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }
    {
        let mesh = asset_server.load("models/char-question.glb#Mesh0/Primitive0");
        // let _material = asset_server.load("models/char-question.glb#Material0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.7, 0.8, 0.6),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(
                R * (3.0 * D_STEP).cos(),
                R * (3.0 * D_STEP).sin(),
                0.0,
            ))
            .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }
    {
        let mesh = asset_server.load("models/char-asterisk.glb#Mesh0/Primitive0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.9, 0.6, 0.8),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(
                R * (4.0 * D_STEP).cos(),
                R * (4.0 * D_STEP).sin(),
                0.0,
            ))
            .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }

    for i in 5..20 {
        let mesh = asset_server.load("models/char-asterisk.glb#Mesh0/Primitive0");
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.9, 0.9, 0.9),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(
                R * (i as f32 * D_STEP).cos(),
                R * (i as f32 * D_STEP).sin(),
                0.0,
            ))
            .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }

    let scale = 3.0;
    for i in 0..5 {
        let mesh = asset_server.load(format!("models/sky.glb#Mesh0/Primitive{}", i).as_str());
        let material = asset_server.load(format!("models/sky.glb#Material{}", i).as_str());
        let transform = Transform::from_scale(Vec3::new(scale, scale, scale))
            .with_translation(Vec3::new(5.0, -2.0, 0.0))
            .with_rotation(Quat::from_rotation_y(-0.4));
        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform,
            ..Default::default()
        });
    }

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
