use sdl2::{
    pixels::Color, rect::Rect
};

use super::position;

pub struct DrawBox {
    //Construct for box
    pub x_offset: f32,
    //x position in window
    pub y_offset: f32,
    //y position in window
    pub box_width: u32,
    //Width of box
    pub box_height: u32,
    //height of box
    pub box_color: Color,
    //color of box
}

impl DrawBox {
    //Implementation of box (all box related functions)
    pub fn new(
        //Create new box object
        x: f32,
        y: f32,
        box_width: u32,
        box_height: u32,
        box_color: Color,
    ) -> Self {
        Self {
            x_offset: x,
            y_offset: y,
            box_width,
            box_height,
            box_color,
        }
    }

    //Draws filled rectangle with specified position, width, height, and color
    pub fn draw(
        &self,
        x: f32,
        y: f32,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let rect = Rect::new(
            (x + self.x_offset + position::get_global_player_x_offset()) as i32,
            (y + self.y_offset + position::get_global_player_y_offset()) as i32,
            self.box_width,
            self.box_height,
        );
        canvas.set_draw_color(self.box_color);
        canvas.fill_rect(rect)
    }

    pub fn draw_no_offset(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let rect = Rect::new(
            self.x_offset as i32,
            self.y_offset as i32,
            self.box_width,
            self.box_height,
        );
        canvas.set_draw_color(self.box_color);
        canvas.fill_rect(rect)
    }
}