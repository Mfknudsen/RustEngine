use sdl2::{pixels::Color, render::WindowCanvas};

use crate::{
    DrawBox, traits::{
        character::Character,
        collider::BoxCollider,
        drawer::Drawer,
        npc::NPC,
        transform::Transform
    }
};

pub struct Flag {
    x: f32,
    y: f32,
    box_x_size: f32,
    box_y_size: f32,
    boxes: Vec<DrawBox>,
}

impl Flag {
    pub(crate) fn new(x_start: f32, y_start: f32) -> Option<Self> {
        if x_start < 0.0 || y_start < 0.0 {
            None
        } else {
            let mut r = Self {
                x: x_start,
                y: y_start,
                box_x_size: 20.0,
                box_y_size: 460.0,
                boxes: Vec::new(),
            };
            r.boxes = r.setup_boxes();

            Some(r)
        }
    }
}

impl Transform for Flag {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, set: f32) {
        self.x = set;
    }

    fn set_y(&mut self, set: f32) {
        self.y = set;
    }

    fn get_x_velocity(&self) -> f32 {
        0.0
    }

    fn get_y_velocity(&self) -> f32 {
        0.0
    }

    fn set_x_velocity(&mut self, _set: f32) {}

    fn set_y_velocity(&mut self, _set: f32) {}

    fn add_force(&mut self, _x: f32, _y: f32) {}
}

impl Drawer for Flag {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_boxes(&self) -> &Vec<DrawBox> {
        &self.boxes
    }

    fn setup_boxes(&self) -> Vec<DrawBox> {
        let mut result = Vec::new();

        // Flag pole
        result.push(DrawBox::new(0.0, 0.0, 20, 460, Color::WHITE));
        result.push(DrawBox::new(-150.0, 30.0, 150, 75, Color::RED));

        result
    }

    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas) {
        for box_obj in &mut self.boxes {
            match box_obj.draw(self.x, self.y, canvas) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

impl BoxCollider for Flag {
    fn move_x(&self) -> f32 {
        0.0
    }

    fn move_y(&self) -> f32 {
        0.0
    }

    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn set_x_position(&mut self, _set: f32) {}

    fn set_y_position(&mut self, _set: f32) {}

    fn x_size(&self) -> f32 {
        self.box_x_size
    }

    fn y_size(&self) -> f32 {
        self.box_y_size
    }

    fn set_x_velocity(&mut self, _set: f32) {}

    fn set_y_velocity(&mut self, _set: f32) {}
}

impl NPC for Flag {}

impl Character for Flag {
    fn update(&mut self) {}

    fn should_remove(&self) -> bool {
        false
    }
}
