use super::blackboard::Blackboard;
use super::result::BTResult;

pub trait BehaviourNode {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BTResult;
}
