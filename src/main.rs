use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseMotion};
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseButton;
use std::sync::{mpsc::channel, Arc, Mutex};

mod agent;
// mod btree;
mod map;
mod messaging;

use agent::{spawn_agents, Agent, AgentConfig, EntityId, Velocity};
use map::{render_map, MapData};
use messaging::{AgentVelocityReceiver, VelocityMsg, VelocityMsgSender};

fn main() {
    // Multi Producer, Single Consumer (MPSC) channel for sending velocities
    let (tx, rx) = channel::<VelocityMsg>();
    let rx = Arc::new(Mutex::new(rx));

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .insert_resource(AgentVelocityReceiver { rx })
        .insert_resource(VelocityMsgSender(tx.clone()))
        .insert_resource(MapData::parse_file("map1.json"))
        .add_systems(Startup, setup)
        .add_systems(Startup, render_map)
        .add_systems(Update, (receive_and_apply_velocity, apply_velocity))
        .add_systems(Update, camera_pan_zoom)
        .run();
// Camera pan and zoom system
use bevy::render::camera::{Camera, Projection, OrthographicProjection};

fn camera_pan_zoom(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
) {
    let mut pan_delta = Vec2::ZERO;
    // Only pan if left mouse button is pressed
    if mouse_button_input.pressed(MouseButton::Left) {
        for event in mouse_motion_events.read() {
            pan_delta += event.delta;
        }
    }

    let mut zoom_delta = 0.0;
    for event in mouse_wheel_events.read() {
        zoom_delta += event.y;
    }

    for (mut transform, mut projection) in &mut query {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            // Pan
            if pan_delta != Vec2::ZERO {
                transform.translation.x -= pan_delta.x * ortho.scale;
                transform.translation.y += pan_delta.y * ortho.scale;
            }
            // Zoom
            if zoom_delta != 0.0 {
                ortho.scale = (ortho.scale * (1.0 - zoom_delta * 0.1)).max(0.1);
            }
        }
    }
}
}

fn setup(mut commands: Commands, tx: Res<VelocityMsgSender>) {
    commands.spawn(Camera2d::default());

    let agent_configs = [
        AgentConfig {
            id: 1,
            initial_position: Vec2::new(0.0, 0.0),
            color: Color::srgb(0.8, 0.2, 0.2),
            time_step: 0.1,
        },
        AgentConfig {
            id: 2,
            initial_position: Vec2::new(0.1, 0.1),
            color: Color::srgb(0.2, 0.8, 0.2),
            time_step: 0.01,
        },
    ];

    spawn_agents(&mut commands, &agent_configs, &tx.0);
}

fn receive_and_apply_velocity(
    rx: ResMut<AgentVelocityReceiver>,
    mut query: Query<(&mut Velocity, &EntityId), With<Agent>>,
) {
    if let Ok(rx) = rx.rx.lock() {
        while let Ok(msg) = rx.try_recv() {
            for (mut velocity, id) in &mut query {
                if id.0 == msg.entity_id.0 {
                    *velocity = msg.velocity.clone();
                }
            }
        }
    }
}

// System to apply velocity to all entities with Velocity and Transform
fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Agent>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
        transform.rotation =
            transform.rotation * Quat::from_rotation_z(velocity.theta_rad * time.delta_secs());
    }
}
