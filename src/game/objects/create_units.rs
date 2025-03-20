use crate::game::maths::pos_2::FIXED_POINT_SCALE;
use crate::game::objects::animation::Animation;
use crate::game::objects::game_object::GameObject;
use crate::game::objects::object_shape::ObjectShape;
use crate::game::objects::object_type::ObjectType;
use std::time::Duration;

pub fn create_enemy_at_point(handle: &str) -> GameObject {
    let animation = Animation::new(handle, Duration::from_secs(1), (20, 20));
    GameObject::new(ObjectType::Enemy, ObjectShape::new(20 * FIXED_POINT_SCALE, 20 * FIXED_POINT_SCALE), 30 * FIXED_POINT_SCALE, 10.0, 10.0, animation)
}