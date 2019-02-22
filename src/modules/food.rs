use maat_graphics::DrawCall;

use crate::modules::maploader::Map;

use cgmath::Vector3;

#[derive(Clone)]
pub struct Food {
  position: Vector3<f32>,
  size: Vector3<f32>,
  model: String,
  tile_number: u32,
  speed: f32,
}

impl Food {
  pub fn new(position: Vector3<f32>, model: String) -> Food {
    Food {
      position,
      size: Vector3::new(1.0, 1.0, 1.0),
      model,
      tile_number: 0,
      speed: 0.0,
    }
  }
  
  pub fn update(&mut self, map: &Map, delta_time: f32) {
    self.speed += delta_time;
    
    if self.speed > 0.5 {
      self.tile_number = map.get_next_path(self.tile_number);
      let map_pos = map.get_path_position(self.tile_number as usize);
      self.position.x = map_pos.x;
      self.position.z = map_pos.y;
      self.speed = 0.0;
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position, Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0), self.model.to_string()));
  }
}
