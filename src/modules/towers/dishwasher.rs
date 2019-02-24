use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::towers::traits::Tower;

use cgmath::{InnerSpace, Angle, Deg, dot, Vector2, Vector3};

#[derive(Clone)]
pub struct Dishwasher {
  position: Vector3<f32>,
  size: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  tile_location: Vector2<u32>,
  range: u32,
  fire_rate: f32,
}

impl Dishwasher {
  pub fn new(position: Vector3<f32>, size: Vector3<f32>, rotation: Vector3<f32>, tile: Vector2<u32>) -> Dishwasher {
    Dishwasher {
      position,
      size,
      rotation,
      model: "Dishwasher".to_string(),
      tile_location: tile,
      range: 3,
      fire_rate: 1.0,
    }
  }
}


impl Tower for Dishwasher {
  fn update(&mut self, foods: &mut Vec<Food>, delta_time: f32) {
    //self.rotation.y += 60.0*delta_time;
    
    for food in foods {
      let location = food.get_tile_location();
      let mag = Vector2::new(location.x as f32-self.tile_location.x as f32, location.y as f32-self.tile_location.y as f32).magnitude();
      /*
      if mag < self.range as f32 {
        let loc = food.get_location();
        let direction = Vector2::new(loc.x-self.position.x, loc.y-self.position.y).normalize();
        let mut angle = Deg::atan2(direction.y, direction.x);
        self.rotation.y = angle.0 as f32+90.0;
      }*/
    }
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
