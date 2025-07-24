use crate::btree::result::BTreeResult;
use crate::btree::blackboard::Blackboard;

pub trait BehaviourNode {
    fn tick(&mut self, blackboard: &mut Blackboard) -> BTreeResult;
}