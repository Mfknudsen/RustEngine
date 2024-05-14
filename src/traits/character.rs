pub trait Character{
    fn update(&mut self);

    fn should_remove(&self) -> bool;
}