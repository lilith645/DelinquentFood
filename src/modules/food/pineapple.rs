use crate::modules::food::{Food, FoodData};
use crate::modules::map::Map;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct Pineapple {
  data: FoodData,
  is_inner: bool,
}

impl Pineapple {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Pineapple {
    let health = 60;
    let speed = 12.5;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(0.5, 0.5, 0.5);
    let sell_price = 3;
    
    Pineapple {
      data: FoodData::new(id, position, rotation, size, speed, health, "Pineapple".to_string(), path.clone(), location, sell_price),
      is_inner: false,
    }
  }
  
  pub fn new_inner(id: i32, position: Vector2<f32>, path: Vec<u32>, path_number: u32, location: Vector2<i32>) -> Pineapple {
    let health = 30;
    let speed = 18.0;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(0.25, 0.25, 0.25);
    let sell_price = 5;
    
    let mut data = FoodData::new(id, position, rotation, size, speed, health, "Pineapple".to_string(), path.clone(), location, sell_price);
    data.path_number = path_number;
    
    Pineapple {
      data,
      is_inner: true,
    }
  }
}

impl Food for Pineapple {
  fn data(&self) -> &FoodData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut FoodData {
    &mut self.data
  }
  
  fn get_children(&self) -> Vec<Box<Food>> {
    if !self.is_inner {
      vec!(Box::new(Pineapple::new_inner(self.data.id, self.data.position.xz(), self.data.path.clone(), self.data.path_number, self.data.path_location)))
    } else {
      Vec::new()
    }
  }
  
  fn local_update(&mut self, map: &Map, delta_time: f32) {
    
  }
}
