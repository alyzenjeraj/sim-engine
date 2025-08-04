use super::blackboard::Blackboard;
use super::node::BehaviourNode;
use super::result::BTResult;

// Runs the Children in Order
pub struct Sequence {
    children: Vec<Box<dyn BehaviourNode>>,
    current: usize,
}

impl Sequence {
    pub fn new(children: Vec<Box<dyn BehaviourNode>>) -> Self {
        Sequence {
            children,
            current: 0,
        }
    }
}
