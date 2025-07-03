use super::components::{Agent, AgentConfig, EntityId, Velocity};
use crate::messaging::types::VelocityMsg;
use bevy::prelude::*;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn spawn_agents(
    commands: &mut Commands,
    agent_configs: &[AgentConfig],
    tx: &Sender<VelocityMsg>,
) {
    for config in agent_configs {
        let tx_child = tx.clone();
        let id = config.id;
        let time_step = config.time_step;

        commands.spawn((
            EntityId(config.id),
            Sprite {
                color: config.color,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            Transform::from_xyz(config.initial_position.x, config.initial_position.y, 0.0),
            GlobalTransform::default(),
            Velocity {
                x: 0.0,
                y: 0.0,
                theta_rad: 0.0,
            },
            Agent,
        ));

        thread::spawn(move || {
            let mut t: f32 = 0.0;

            loop {
                let vx = 100.0 * t.cos();
                let vy = 50.0 * t.sin();

                tx_child
                    .send(VelocityMsg {
                        entity_id: EntityId(id),
                        velocity: Velocity {
                            x: vx,
                            y: vy,
                            theta_rad: 0.1, // - test constant rotational speed
                        },
                    })
                    .unwrap();

                t += time_step;
                std::thread::sleep(Duration::from_millis(50));
            }
        });
    }
}
