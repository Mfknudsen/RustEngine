pub trait Transform{
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn add_force(&mut self, x: f32, y:f32);
}