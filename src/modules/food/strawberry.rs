use crate::modules::food::{Food, FoodData};
use crate::modules::map::Map;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct Strawberry {
  data: FoodData,
}

impl Strawberry {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Strawberry {
    let health = 80;
    let speed = 10.0;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(1.5, 1.5, 1.5);
    Strawberry {
      data: FoodData::new(id, position, rotation, size, speed, health, "Strawberry".to_string(), path.clone(), location),
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
  
  fn local_update(&mut self, map: &Map, delta_time: f32) {
    self.data.rotation.y += 90.0*delta_time;//angle.0 as f32+90.0;
    self.data.total_dt += delta_time*0.5;
    if self.data.total_dt > 3.14 {
      self.data.total_dt -= 3.14;
    }
    self.data.position.y = 1.0 + 2.0*self.data().total_dt.sin();
  }
}
