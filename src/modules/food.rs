use maat_graphics::DrawCall;

use crate::modules::map::Map;

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3};

#[derive(Clone)]
pub struct Food {
  id: i32,
  position: Vector3<f32>,
  size: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  path_number: u32,
  path_location: Vector2<i32>,
  speed: f32,
  target: Vector2<f32>,
  path: Vec<u32>,
  health: i32,
}

impl Food {
  pub fn new(id: i32, position: Vector3<f32>, health: i32, model: String, path: Vec<u32>, location: Vector2<i32>) -> Food {
    Food {
      id,
      position,
      size: Vector3::new(1.0, 1.0, 1.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      model,
      path_number: 0,
      path_location: location,
      speed: 10.0,
      target: position.xz(),
      path,
      health,
    }
  }
  
  pub fn update(&mut self, map: &Map, delta_time: f32) {
    if (self.position.x-self.target.x + self.position.z-self.target.y).abs() < 0.1 {
      self.path_number += 1;
      if self.path_number >= self.path.len() as u32 {
        self.path_number = 0;
      }
      let map_pos = map.tile_position_from_index(self.path[self.path_number as usize] as usize);
      
      self.path_location = map.get_qr_from_index(self.path[self.path_number as usize] as usize);
      
      self.target.x = map_pos.x;
      self.target.y = map_pos.y;
    }
    
    let direction = Vector2::new(self.target.x-self.position.x, self.target.y-self.position.z).normalize();
    let angle = Deg::atan2(direction.x, direction.y);
    
    self.rotation.y = angle.0 as f32+90.0;
    self.position.x += direction.x*self.speed*delta_time;
    self.position.z += direction.y*self.speed*delta_time;
  }
  
  pub fn get_id(&self) -> i32 {
    self.id
  }
  
  pub fn is_cooked(&self) -> bool {
    self.health <= 0
  }
  
  pub fn apply_damage(&mut self, dmg: i32) {
    self.health -= dmg;
  }
  
  pub fn get_tile_location(&self) -> Vector2<i32> {
    self.path_location
  }
  
  pub fn get_location(&self) -> Vector2<f32> {
    self.position.xz()
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position, Vector3::new(2.0, 2.0, 2.0), self.rotation, self.model.to_string()));
  }
}
