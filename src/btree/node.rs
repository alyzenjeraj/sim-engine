use super::result::BTResult;
use super::blackboard::Blackboard;

pub trait BehaviourNode {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BTResult;
}