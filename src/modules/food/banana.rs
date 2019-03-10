use crate::modules::food::{Food, FoodData};
use crate::modules::map::Map;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct Banana {
  data: FoodData,
}

impl Banana {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Banana {
    let health = 20;
    let speed = 15.0;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(1.0, 1.0, 1.0);
    let sell_price = 5;
    Banana {
      data: FoodData::new(id, position, rotation, size, speed, health, "Bombard".to_string(), path.clone(), location, sell_price),
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
  
  fn local_update(&mut self, map: &Map, delta_time: f32) {
    
  }
}
