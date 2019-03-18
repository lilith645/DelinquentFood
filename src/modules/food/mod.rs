pub use self::strawberry::Strawberry;
pub use self::banana::Banana;
pub use self::cake::Cake;
pub use self::pineapple::Pineapple;
pub use self::mushroom::Mushroom;
pub use self::jelly::Jelly;

mod strawberry;
mod banana;
mod cake;
mod pineapple;
mod mushroom;
mod jelly;

use maat_graphics::DrawCall;

use crate::modules::weapons::Debuff;
use crate::modules::map::Map;

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3};

#[derive(Clone)]
pub struct FoodData {
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
  max_health: i32,
  total_dt: f32,
  cooked: bool,
  rotten: bool,
  sell_price: i32,
}

impl FoodData {
  pub fn new(id: i32, position: Vector3<f32>, rotation: Vector3<f32>, size: Vector3<f32>, speed: f32, health: i32, model: String, path: Vec<u32>, location: Vector2<i32>, sell_price: i32) -> FoodData {
    FoodData {
      id,
      position,
      size,
      rotation,
      model,
      debuffs: Vec::new(),
      path_number: 0,
      path_location: location,
      speed,
      target: position.xz(),
      path,
      health,
      max_health: health,
      total_dt: 0.0,
      cooked: false,
      rotten: false,
      sell_price,
    }
  }
}

pub trait FoodClone {
  fn clone_food(&self) -> Box<Food>;
}

impl<T: 'static + Food + Clone> FoodClone for T {
  fn clone_food(&self) -> Box<Food> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Food> {
  fn clone(&self) -> Box<Food> {
    self.clone_food()
  }
}

pub trait Food: FoodClone {
  fn data(&self) -> &FoodData;
  fn mut_data(&mut self) -> &mut FoodData;
  fn get_bin_space(&self) -> i32;
  fn get_children(&self, map: &Map) -> Vec<Box<Food>>;
  
  fn local_update(&mut self, map: &Map, move_angle: f32, delta_time: f32);
  fn update(&mut self, map: &Map, delta_time: f32) {
    if self.data().health > self.data().max_health {
      self.mut_data().max_health = self.data().health;
    }
    
    if (self.data().position.x-self.data().target.x + self.data().position.z-self.data().target.y).abs() < 0.4 {
      self.mut_data().path_number += 1;
      if self.data().path_number >= self.data().path.len() as u32 {
        self.mut_data().rotten = true;
        self.mut_data().cooked = false;
        return;
      }
      
      let map_pos = map.tile_position_from_index(self.data().path[self.data().path_number as usize] as usize);
      
      self.mut_data().path_location = map.get_qr_from_index(self.data().path[self.data().path_number as usize] as usize);
      
      self.mut_data().target.x = map_pos.x;
      self.mut_data().target.y = map_pos.y;
    }
    
    let direction = Vector2::new(self.data().target.x-self.data().position.x, self.data().target.y-self.data().position.z).normalize();
    let angle = Deg::atan2(direction.x, direction.y);
    
    let mut speed = self.data().speed;
    let mut remove_debuffs = Vec::new();
    for i in 0..self.data().debuffs.len() {
      match &mut self.mut_data().debuffs[i] {
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
      self.mut_data().debuffs.remove(remove_debuffs[i]-offset);
      offset += 1;
    }
    
    direction.normalize();
    self.mut_data().position.x += direction.x*speed*delta_time;
    self.mut_data().position.z += direction.y*speed*delta_time;
    
    self.local_update(map, angle.0, delta_time);
  }
  
  fn get_health(&self) -> i32 {
    self.data().health
  }
  
  fn get_path_num(&self) -> u32 {
    self.data().path_number
  }
  
  fn get_id(&self) -> i32 {
    self.data().id
  }
  
  fn is_cooked(&self) -> bool {
    self.data().cooked
  }
  
  fn is_rotten(&self) -> bool {
    self.data().rotten && !self.is_cooked()
  }
  
  fn apply_damage(&mut self, dmg: i32) {
    self.mut_data().health -= dmg;
    if self.data().health <= 0 {
      self.mut_data().cooked = true;
    }
   // println!("id: {}, health: {}", self.data().id, self.data().health);
  }
  
  fn apply_debuffs(&mut self, debuffs: Vec<Debuff>) {
    for debuff in debuffs {
      if !self.data().debuffs.contains(&debuff) {
        self.mut_data().debuffs.push(debuff);
      }
    }
  }
  
  fn sell_price(&self) -> i32 {
    self.data().sell_price
  }
  
  fn get_tile_location(&self) -> Vector2<i32> {
    self.data().path_location
  }
  
  fn get_location(&self) -> Vector2<f32> {
    self.data().position.xz()
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    //draw_calls.push(DrawCall::draw_model(self.data().position, self.data().size, self.data().rotation, self.data().model.to_string()));
    //draw_calls.push(DrawCall::add_instanced_model(self.data().model.to_string(), self.data().position, self.data().size, self.data().rotation));
    draw_calls.push(DrawCall::add_instanced_model_overwrite_colour(self.data().model.to_string(), self.data().position, self.data().size, self.data().rotation, Vector3::new(1.0 - (self.data().health as f32/self.data().max_health as f32), self.data().health as f32/self.data().max_health as f32, 0.0)));
  }
}

