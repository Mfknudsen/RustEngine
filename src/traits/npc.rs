use crate::{
    traits::{
        character::Character,
        transform::Transform,
        collider::BoxCollider,
        drawer::Drawer
    }
};

pub trait NPC: Character + Transform + BoxCollider+ Drawer{

}