use crate::traits::character::Character;
use crate::traits::transform::Transform;
use crate::traits::collider::BoxCollider;
use crate::traits::drawer::Drawer;
pub trait NPC: Character + Transform + BoxCollider+ Drawer{

}