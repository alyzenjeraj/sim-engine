use crate::agent::components::{EntityId, Velocity};
use bevy::prelude::*;
use std::sync::{mpsc::Receiver, mpsc::Sender, Arc, Mutex};

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
