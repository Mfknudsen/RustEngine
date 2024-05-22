use sdl2::render::WindowCanvas;
use crate::DrawBox;

pub trait Drawer {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;

    fn get_boxes(&self) -> &Vec<DrawBox>;

    fn setup_boxes(&self) -> Vec<DrawBox>;

    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas) {
        let x = self.get_x();
        let y = self.get_y();

        for box_obj in self.get_boxes().iter() {
            match box_obj.draw(x, y, canvas) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error: {}", e);
                }
            }        }
    }
}