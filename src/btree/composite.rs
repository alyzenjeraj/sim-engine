use super::node::BehaviorNode;
use super::result::BTResult;
use super::blackboard::Blackboard;

pub struct Sequence {
    children: Vec<Box<dyn BehaviorNode>>,
    current: usize,
}