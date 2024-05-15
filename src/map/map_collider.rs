use crate::traits::collider::BoxCollider;

pub struct MapCollider {
    x: f32,
    y: f32,
    x_size: f32,
    y_size: f32,
}

impl MapCollider {
    pub(crate) fn new(x: f32, y: f32, x_size: f32, y_size: f32) -> Self {
        Self {
            x,
            y,
            x_size,
            y_size,
        }
    }
}

impl BoxCollider for MapCollider {
    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn x_size(&self) -> f32 {
        self.x_size
    }

    fn y_size(&self) -> f32 {
        self.y_size
    }
}

impl BoxCollider for & MapCollider {
    fn x_position(&self) -> f32 {
        self.x
    }

    fn y_position(&self) -> f32 {
        self.y
    }

    fn x_size(&self) -> f32 {
        self.x_size
    }

    fn y_size(&self) -> f32 {
        self.y_size
    }

    fn check_against_map(&mut self, _map_colliders: &mut Vec<MapCollider>) {
        //Map shouldn't check against map
    }

    fn point_in_box(&self, x: i32, y: i32) -> bool {
        x > self.x as i32 &&
            y > self.y as i32 &&
            x < (self.x + self.x_size()) as i32 &&
            y < (self.y + self.y_size()) as i32
    }
}