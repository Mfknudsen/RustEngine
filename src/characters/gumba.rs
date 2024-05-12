use sdl2::render::WindowCanvas;
use crate::DrawBox;
use crate::map::map_collider::MapCollider;
use crate::traits::collider::BoxCollider;
use crate::traits::drawer::Drawer;
use crate::traits::transform::Transform;

const TURTLE_MOVE_SPEED: f32 = 500.0;

pub struct Gumba {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    box_x_size: f32,
    box_y_size: f32,
    boxes: Vec<DrawBox>,
    grounded: bool,
}

impl Gumba {
    pub(crate) fn new(x_start: f32, y_start: f32) -> Self {
        Self {
            x: x_start,
            y: y_start,
            x_velocity: 0.0,
            y_velocity: 0.0,
            box_x_size: 50.0,
            box_y_size: 50.0,
            grounded: false,
            boxes: Self::setup_boxes(),
        }
    }

    fn setup_boxes() -> Vec<DrawBox>{
        let mut result = Vec::new();

        return result
    }
}

impl Transform for Gumba {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn add_force(&mut self, x: f32, y: f32) {
    }
}

impl Drawer for Gumba {
    fn draw_on_canvas(&mut self, canvas: &mut WindowCanvas) {
     }
}

impl BoxCollider for Gumba {
    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn x_size(&self) -> f32 {
        self.box_x_size
    }

    fn y_size(&self) -> f32 {
        self.box_y_size
    }

    fn x_center(&self) -> f32 {
        todo!()
    }

    fn y_center(&self) -> f32 {
        todo!()
    }

    fn check_against_map(&self, map_colliders: &mut Vec<MapCollider>) {
        todo!()
    }
}
