use maat_graphics::DrawCall;

use crate::modules::weapons::Debuff;
use crate::modules::map::Map;

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3};

#[derive(Clone)]
pub struct Food {
  id: i32,
  position: Vector3<f32>,
  size: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  debuffs: Vec<Debuff>,
  path_number: u32,
  path_location: Vector2<i32>,
  speed: f32,
  target: Vector2<f32>,
  path: Vec<u32>,
  health: i32,
  total_dt: f32,
  cooked: bool,
}

impl Food {
  pub fn new(id: i32, position: Vector3<f32>, health: i32, model: String, path: Vec<u32>, location: Vector2<i32>) -> Food {
    Food {
      id,
      position,
      size: Vector3::new(1.0, 1.0, 1.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      model,
      debuffs: Vec::new(),
      path_number: 0,
      path_location: location,
      speed: 10.0,
      target: position.xz(),
      path,
      health,
      total_dt: 0.0,
      cooked: false,
    }
  }
  
  pub fn update(&mut self, map: &Map, delta_time: f32) {
    if (self.position.x-self.target.x + self.position.z-self.target.y).abs() < 0.1 {
      self.path_number += 1;
      if self.path_number >= self.path.len() as u32 {
        self.health = 0;
        self.cooked = false;
        return;
      }
      
      let map_pos = map.tile_position_from_index(self.path[self.path_number as usize] as usize);
      
      self.path_location = map.get_qr_from_index(self.path[self.path_number as usize] as usize);
      
      self.target.x = map_pos.x;
      self.target.y = map_pos.y;
    }
    
    let direction = Vector2::new(self.target.x-self.position.x, self.target.y-self.position.z).normalize();
    let angle = Deg::atan2(direction.x, direction.y);
    
    let mut speed = self.speed;
    let mut remove_debuffs = Vec::new();
    for i in 0..self.debuffs.len() {
      match &mut self.debuffs[i] {
        Debuff::Slow(timer) => {
          *timer -= delta_time;
          if *timer <= 0.0 {
            remove_debuffs.push(i);
          } else {
            speed *= 0.65;
          }
        },
        Debuff::Freeze(timer) => {
          speed = 0.0;
        },
        Debuff::Reverse(timer) => {
          speed = -speed;
        }
      }
    }
    
    let mut offset = 0;
    for i in 0..remove_debuffs.len() {
      self.debuffs.remove(remove_debuffs[i]-offset);
      offset += 1;
    }
    
    self.rotation.y += 90.0*delta_time;//angle.0 as f32+90.0;
    self.position.x += direction.x*speed*delta_time;
    self.position.z += direction.y*speed*delta_time;
    self.position.y = 1.0 + 2.0*self.total_dt.sin();
    
    self.total_dt += delta_time*0.5;
    if self.total_dt > 3.14 {
      self.total_dt -= 3.14;
    }
  }
  
  pub fn get_id(&self) -> i32 {
    self.id
  }
  
  pub fn is_cooked(&self) -> bool {
    self.cooked
  }
  
  pub fn is_rotten(&self) -> bool {
    self.health <= 0 && !self.is_cooked()
  }
  
  pub fn apply_damage(&mut self, dmg: i32) {
    self.health -= dmg;
    if self.health <= 0 {
      self.cooked = true;
    }
  }
  
  pub fn apply_debuffs(&mut self, debuffs: Vec<Debuff>) {
    for debuff in debuffs {
      if !self.debuffs.contains(&debuff) {
        self.debuffs.push(debuff);
      }
    }
  }
  
  pub fn get_tile_location(&self) -> Vector2<i32> {
    self.path_location
  }
  
  pub fn get_location(&self) -> Vector2<f32> {
    self.position.xz()
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position, self.size, self.rotation, self.model.to_string()));
  }
}
