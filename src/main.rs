use bevy::prelude::*;

// Velocity component for x, y, and theta (rotation) velocities
#[derive(Component)]
struct Velocity {
    pub x: f32,
    pub y: f32,
    pub theta: f32, // radians per second
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, apply_velocity)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Velocity { x: -50.0, y: 0.0, theta: 0.5 }, // Example: move left and rotate
    ));
}

// System to apply velocity to all entities with Velocity and Transform
fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
        transform.rotation = transform.rotation * Quat::from_rotation_z(velocity.theta * time.delta_secs());
    }
}
