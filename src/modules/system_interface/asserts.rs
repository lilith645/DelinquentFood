
use cgmath::{Vector2, Vector4};

pub fn check_colour_range_vec4(colour: &Vector4<f32>) {
  debug_assert!(!(colour.x < 0.0 || colour.x > 1.0), "Red colour must be with 0.0 and 1.0, actual: {}", colour.x);
  debug_assert!(!(colour.y < 0.0 || colour.y > 1.0), "Green colour must be with 0.0 and 1.0, actual: {}", colour.y);
  debug_assert!(!(colour.z < 0.0 || colour.z > 1.0), "Blue colour must be with 0.0 and 1.0, actual: {}", colour.z);
  debug_assert!(!(colour.y < 0.0 || colour.y > 1.0), "Aplha colour must be with 0.0 and 1.0, actual: {}", colour.w);
}

pub fn vec2_greater_than<'a>(value: &Vector2<f32>, some_value: &'a f32, message: &'a str) {
  debug_assert!(&value.x > some_value, message.to_string());
  debug_assert!(&value.y > some_value, message.to_string());
}

pub fn _check_greater_than<'a>(value: &f32, some_value: &f32, message: &'a str) {
   debug_assert!(value > some_value, message.to_string());
}
