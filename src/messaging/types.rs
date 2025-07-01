use bevy::prelude::*;
use std::sync::{Arc, Mutex, mpsc::Receiver, mpsc::Sender};
use crate::agent::components::{EntityId, Velocity};

pub struct VelocityMsg {
    pub entity_id: EntityId,
    pub velocity: Velocity,
}

#[derive(Resource)]
pub struct AgentVelocityReceiver {
    pub rx: Arc<Mutex<Receiver<VelocityMsg>>>,
}

#[derive(Resource, Clone)]
pub struct VelocityMsgSender(pub Sender<VelocityMsg>);