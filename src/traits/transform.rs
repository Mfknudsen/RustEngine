pub trait Transform{
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;

    fn set_x(&mut self, set: f32);
    fn set_y(&mut self, set: f32);

    fn get_x_velocity(&self) -> f32;
    fn get_y_velocity(&self) -> f32;

    fn set_x_velocity(&mut self, set: f32);
    fn set_y_velocity(&mut self, set: f32);

    fn add_force(&mut self, x: f32, y:f32);
}