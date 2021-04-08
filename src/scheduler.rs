pub trait Scheduler {
    fn next(&mut self);
    fn execute(&mut self);
}
