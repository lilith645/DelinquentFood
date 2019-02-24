use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::modules::maploader::Map;

use cgmath::{InnerSpace, Angle, Deg, dot, Vector2, Vector3};

#[derive(Clone)]
pub struct Food {
  position: Vector3<f32>,
  size: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  tile_number: u32,
  tile_location: Vector2<u32>,
  speed: f32,
  target: Vector2<f32>,
}

impl Food {
  pub fn new(position: Vector3<f32>, model: String) -> Food {
    Food {
      position,
      size: Vector3::new(1.0, 1.0, 1.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      model,
      tile_location: Vector2::new(0, 0),
      tile_number: 1,
      speed: 10.0,
      target: position.xz(),
    }
  }
  
  pub fn update(&mut self, map: &Map, delta_time: f32) {
    /*if (self.position.x-self.target.x + self.position.z-self.target.y).abs() < 0.1 {
      self.tile_number = map.get_next_path(self.tile_number);
      let map_pos = map.get_path_position(self.tile_number as usize);
      self.tile_location = map.get_path_location(self.tile_number as usize);
      self.target.x = map_pos.x;
      self.target.y = map_pos.y;
    }*/
    
    let direction = Vector2::new(self.target.x-self.position.x, self.target.y-self.position.z).normalize();
    let angle = Deg::atan2(direction.x, direction.y);
    
    self.rotation.y = angle.0 as f32+90.0;
    self.position.x += direction.x*self.speed*delta_time;
    self.position.z += direction.y*self.speed*delta_time;
  }
  
  pub fn get_tile_location(&self) -> Vector2<u32> {
    self.tile_location
  }
  
  pub fn get_location(&self) -> Vector2<f32> {
    self.position.xz()
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position, Vector3::new(1.0, 1.0, 1.0), self.rotation, self.model.to_string()));
  }
}
