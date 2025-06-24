use crate::agents::agent::AgentPosition;
use std::sync::mpsc::Receiver;

pub struct AgentPositionReceiver {
    pub rx: Receiver<AgentPosition>,
}
