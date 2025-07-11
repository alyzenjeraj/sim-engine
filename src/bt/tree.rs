pub enum Status {
    Success,
    Failure,
    Running,
}

pub trait BehaviorNode {
    fn tick(&mut self) -> Status;
}