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

    {
        let mesh0 = asset_server.load("models/sky.glb#Mesh0/Primitive0");
        let mesh1 = asset_server.load("models/sky.glb#Mesh0/Primitive1");
        let mesh2 = asset_server.load("models/sky.glb#Mesh0/Primitive2");
        let mesh3 = asset_server.load("models/sky.glb#Mesh0/Primitive3");
        let mesh4 = asset_server.load("models/sky.glb#Mesh0/Primitive4");
        let material0 = materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            ..Default::default()
        });
        let material1 = materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.6, 0.5),
            ..Default::default()
        });
        let material2 = materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.3, 0.3),
            ..Default::default()
        });
        let transform = Transform::from_scale(Vec3::new(1.0, 1.0, 1.0))
            .with_translation(Vec3::new(2.0, 1.0, 0.0));
        // .with_rotation(Quat::from_rotation_x(PI_2));
        commands.spawn_bundle(PbrBundle {
            mesh: mesh0,
            material: material0,
            transform: transform.clone(),
            ..Default::default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: mesh1,
            material: material1,
            transform: transform.clone(),
            ..Default::default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: mesh2,
            material: material2,
            transform,
            ..Default::default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: mesh3,
            // material: material2,
            transform,
            ..Default::default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: mesh4,
            // material: material2,
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
