use super::node::BehaviourNode;
use super::result::BTResult;
use super::blackboard::Blackboard;

pub struct Sequence {
    children: Vec<Box<dyn BehaviourNode>>,
    current: usize,
}