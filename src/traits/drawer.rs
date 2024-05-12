use sdl2::render::WindowCanvas;

pub trait Drawer {
    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas);
}