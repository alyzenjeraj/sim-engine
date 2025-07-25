#[derive(Debug)]
pub struct Blackboard {
    pub has_goal: bool,
    pub at_goal: bool,
    pub path: Vec<usize>,
    pub path_index: usize,
    AgentId: usize,
}