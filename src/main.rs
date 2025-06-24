use bevy::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Component)]
struct AgentSprite;

#[derive(Debug, Clone, Copy)]
struct AgentPosition {
    x: f32,
    y: f32,
}

#[derive(Resource)]
struct AgentPositionReceiver {
    rx: Receiver<AgentPosition>,
}

fn main() {
    let (tx, rx) = channel();

    // Spawn the agent thread
    thread::spawn(move || {
        let mut x = -300.0;
        let y = 0.0;

        loop {
            x += 1.0;
            if x > 300.0 {
                x = -300.0;
            }

            tx.send(AgentPosition { x, y }).unwrap();
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }
    });

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AgentPositionReceiver { rx })
        .add_systems(Startup, setup)
        .add_systems(Update, update_agent_position)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            ..default()
        },
        AgentSprite,
    ));
}

fn update_agent_position(
    mut query: Query<&mut Transform, With<AgentSprite>>,
    rx: Res<AgentPositionReceiver>,
) {
    if let Ok(position) = rx.rx.try_recv() {
        if let Ok(mut transform) = query.get_single_mut() {
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}
