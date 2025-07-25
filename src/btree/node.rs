use super::result::BTreeResult;
use super::blackboard::Blackboard;

pub trait BehaviourNode {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BTreeResult;
}