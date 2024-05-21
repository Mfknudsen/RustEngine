use sdl2::{
    pixels::Color,
    render::WindowCanvas,
};

use crate::{
    DrawBox,
    get_delta_time,
    traits::character::Character,
    traits::collider::BoxCollider,
    traits::drawer::Drawer,
    traits::transform::Transform,
    traits::npc::NPC,
};

const GUMBA_MOVE_SPEED: f32 = 250.0;

pub struct Gumba {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    box_x_size: f32,
    box_y_size: f32,
    boxes: Vec<DrawBox>,
    walk_direction: f32,
    dead: bool,
}

impl Gumba {
    pub(crate) fn new(x_start: f32, y_start: f32) -> Result<Self, &'static str> {
        if x_start < 0.0 || y_start < 0.0 {
            Err("Value cannot be negative")
        }
        else {
            Ok(
                Self {
                    x: x_start,
                    y: y_start,
                    x_velocity: 0.0,
                    y_velocity: 0.0,
                    box_x_size: 50.0,
                    box_y_size: 50.0,
                    boxes: Self::setup_boxes(),
                    walk_direction: -1.0,
                    dead: false,
                }
            )
        }
    }

    fn setup_boxes() -> Vec<DrawBox> {
        let mut result = Vec::new();

        result.push(DrawBox::new(0.0, 0.0, 50, 50, Color::GREEN));

        return result;
    }
}

impl Transform for Gumba {
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
        self.x_velocity
    }

    fn get_y_velocity(&self) -> f32 {
        self.y_velocity
    }

    fn set_x_velocity(&mut self, set: f32) {
        self.x_velocity = set;
    }

    fn set_y_velocity(&mut self, set: f32) {
        self.y_velocity = set;
    }

    fn add_force(&mut self, x: f32, y: f32) {
        self.x_velocity += x;
        self.y_velocity += y;
    }
}

impl Drawer for Gumba {
    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas) {
        for box_obj in &mut self.boxes {
            box_obj.draw(self.x, self.y, canvas).expect("ERROR");
        }
    }
}

impl BoxCollider for Gumba {
    fn move_x(&self) -> f32 {
        self.x_velocity + self.walk_direction * GUMBA_MOVE_SPEED * get_delta_time()
    }

    fn move_y(&self) -> f32 {
        self.y_velocity
    }

    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn set_x_position(&mut self, set: f32) {
        self.x = set;
    }

    fn set_y_position(&mut self, set: f32) {
        self.y = set;
    }

    fn x_size(&self) -> f32 {
        self.box_x_size
    }

    fn y_size(&self) -> f32 {
        self.box_y_size
    }

    fn set_x_velocity(&mut self, set: f32) {
        self.x_velocity = set;
    }

    fn set_y_velocity(&mut self, set: f32) {
        self.y_velocity = set;
    }
}
impl NPC for Gumba{
}
impl Character for Gumba {
    fn update(&mut self) {
        self.x += self.walk_direction * GUMBA_MOVE_SPEED * get_delta_time();
        self.x += self.x_velocity * get_delta_time();
        self.y += self.y_velocity * get_delta_time();
    }

    fn should_remove(&self) -> bool {
        self.dead
    }
}
