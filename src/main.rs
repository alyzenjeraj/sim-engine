use bevy::prelude::*;
use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Component)]
struct Agent;

// Velocity component for x, y, and theta (rotation) velocities
#[derive(Component, Clone)]
struct Velocity {
    pub x: f32,
    pub y: f32,
    pub theta: f32, // radians per second
}

#[derive(Component)]
struct EntityId(i32);

struct VelocityMsg {
    entity_id: EntityId,
    velocity: Velocity,
}

#[derive(Resource)]
struct AgentVelocityReceiver {
    pub rx: Arc<Mutex<Receiver<VelocityMsg>>>,
}   




fn main() {
    // Multi Producer, Single Consumer (MPSC) channel for sending velocities
    let (tx, rx) : (Sender<VelocityMsg>, Receiver<VelocityMsg>) = channel();
    let rx = Arc::new(Mutex::new(rx));

    // Spawn single thread to temporarily simulate velocity updates
    thread::spawn(move || {
        let mut t : f32 = 0.0;


        loop {
            let vx = -100.0 * t.cos(); 
            let vy = 50.0 * t.sin();

            tx.send(VelocityMsg{entity_id: EntityId(1),velocity: Velocity {
                x: vx,
                y: vy,
                theta: 0.1, // - test constant rotational speed
            }}).unwrap();

            t += 0.01;
            std::thread::sleep(Duration::from_millis(50)); // Simulate some delay
        }
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .insert_resource(AgentVelocityReceiver { rx })
        .add_systems(Startup, setup)
        .add_systems(Update, (receive_and_apply_velocity, apply_velocity))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        EntityId(1), 
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Velocity { x: 50.0, y: 10.0, theta: 0.5 }, // Example: move left and rotate
        Agent, // Add Agent component - marker to signify the roles of this entity
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.4),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Velocity { x: 50.0, y: 10.0, theta: 0.5 }, // Example: move left and rotate
    ));
}

fn receive_and_apply_velocity(
    rx: ResMut<AgentVelocityReceiver>,
    mut query: Query<(&mut Velocity, &EntityId), With<Agent>>,
) {
    if let Ok(rx) = rx.rx.lock() {
    while let Ok(msg) = rx.try_recv() {
        for (mut velocity, id) in &mut query {
            if id.0 == msg.entity_id.0 {
                *velocity = msg.velocity.clone(); // Update the velocity of the entity with matching ID
            }
        }
    }}
}

// System to apply velocity to all entities with Velocity and Transform
fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Agent>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
        transform.rotation = transform.rotation * Quat::from_rotation_z(velocity.theta * time.delta_secs());
    }
}
