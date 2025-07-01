use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub theta_rad: f32, // radians per second
}

#[derive(Component)]
pub struct EntityId(pub i32);

pub struct AgentConfig {
    pub id: i32,
    pub initial_position: Vec2,
}