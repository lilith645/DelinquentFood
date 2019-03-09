use crate::modules::food::{Food, FoodData};

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct Strawberry {
  data: FoodData,
}

impl Strawberry {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Strawberry {
    let health = 20;
    Strawberry {
      data: FoodData::new(id, Vector3::new(position.x, 0.0, position.y), health, "Strawberry".to_string(), path.clone(), location),
    }
  }
}

impl Food for Strawberry {
  fn data(&self) -> &FoodData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut FoodData {
    &mut self.data
  }
}
