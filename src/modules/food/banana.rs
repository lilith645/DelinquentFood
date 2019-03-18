use crate::modules::food::{Food, FoodData};
use crate::modules::map::Map;

use cgmath::{Vector2, Vector3};

const MAX_HEALTH: i32 = 30;

#[derive(Clone)]
pub struct Banana {
  data: FoodData,
}

impl Banana {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Banana {
    let health = MAX_HEALTH;
    let speed = 15.0;
    let position = Vector3::new(position.x, 5.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(0.5, 0.5, 0.5);
    let sell_price = 2;
    Banana {
      data: FoodData::new(id, position, rotation, size, speed, health, "Banana".to_string(), path.clone(), location, sell_price)
    }
  }
}

impl Food for Banana {
  fn data(&self) -> &FoodData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut FoodData {
    &mut self.data
  }
  
  fn get_bin_space(&self) -> i32 {
    println!("{} / {} = {}", self.data().health as f32, MAX_HEALTH as f32, (self.data().health as f32 / MAX_HEALTH as f32));
    (5.0 * (self.data().health as f32 / MAX_HEALTH as f32)).ceil() as i32
  }
  
  fn get_children(&self, map: &Map) -> Vec<Box<Food>> {
    Vec::new()
  }
  
  fn local_update(&mut self, map: &Map, move_angle: f32, delta_time: f32) {
    self.data.rotation.y += 90.0*delta_time;//angle.0 as f32+90.0;
  }
}
