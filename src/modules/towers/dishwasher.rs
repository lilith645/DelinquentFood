use maat_graphics::DrawCall;

use crate::modules::towers::traits::Tower;

use cgmath::{Vector3};

#[derive(Clone)]
pub struct Dishwasher {
  position: Vector3<f32>,
  size: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  fire_rate: f32,
  range: u32,
}

impl Dishwasher {
  pub fn new(position: Vector3<f32>, size: Vector3<f32>, rotation: Vector3<f32>) -> Dishwasher {
    Dishwasher {
      position,
      size,
      rotation,
      model: "Dishwasher".to_string(),
      fire_rate: 1.0,
      range: 3,
    }
  }
}


impl Tower for Dishwasher {
  fn update(&mut self, delta_time: f32) {
    self.rotation.y += 60.0*delta_time;
  }
  
  fn fire(&mut self) {
    
  }
  
  fn upgrade(&mut self) {
    
  }
  
  fn apply_effect(&self) {
    
  }
  
  fn remove_effects(&self) {
    
  }
  
  fn upgrade_cost(&self) -> u32 {
    1
  }
  
  fn sell(&self) -> u32 {
    1
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position, self.size, self.rotation, self.model.to_string()));
  }
}
