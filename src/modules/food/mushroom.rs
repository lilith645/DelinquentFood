use crate::modules::food::{Food, FoodData};
use crate::modules::map::Map;

use cgmath::{Vector2, Vector3};

const GROW_SPEED: f32 = 0.15;
const MAX_SIZE: f32 = 3.0;

const START_SPEED: f32 = 40.0;
const START_HEALTH: i32 = 10;

#[derive(Clone)]
pub struct Mushroom {
  data: FoodData,
  grow_speed: f32,
  last_size_stage: f32,
}

impl Mushroom {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Mushroom {
    let health = START_HEALTH;
    let speed = START_SPEED;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(0.5, 0.5, 0.5);
    let sell_price = 2;
    Mushroom {
      data: FoodData::new(id, position, rotation, size, speed, health, "Mushroom".to_string(), path.clone(), location, sell_price),
      grow_speed: GROW_SPEED,
      last_size_stage: 0.5,
    }
  }
  
  pub fn new_baby_shroom(id: i32, position: Vector2<f32>, path: Vec<u32>, path_number: u32, location: Vector2<i32>) -> Mushroom {
    let health = START_HEALTH;
    let speed = START_SPEED;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, 0.0, 0.0);
    let size = Vector3::new(0.5, 0.5, 0.5);
    let sell_price = 2;
    
    let mut data = FoodData::new(id, position, rotation, size, speed, health, "Mushroom".to_string(), path.clone(), location, sell_price);
    data.path_number = path_number;
    
    Mushroom {
      data,
      grow_speed: GROW_SPEED,
      last_size_stage: 0.5,
    }
  }
}

impl Food for Mushroom {
  fn data(&self) -> &FoodData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut FoodData {
    &mut self.data
  }
  
  fn get_children(&self, map: &Map) -> Vec<Box<Food>> {
    let mut shroom_childs: Vec<Box<Food>> = Vec::new();
    
    let size = self.data().size.x;
    let num_shrooms = (size/0.6).floor() as u32;
    
    println!("Num Shrooms: {}", num_shrooms);
    
    let half_shrooms = (num_shrooms as f32*0.5).floor() as usize;
    
    for i in 0..num_shrooms as usize {
      let path_num;
      if i < half_shrooms {
        if self.data().path_number <= i as u32 {
          path_num = 0;
        } else {
          path_num = self.data().path_number-i as u32-1;
        }
      } else {
        if self.data().path_number+(i as u32 - half_shrooms as u32) >= (self.data().path.len()-1) as u32 {
          path_num = (self.data().path.len()-1) as u32;
        } else {
          path_num = self.data().path_number+(i as u32-half_shrooms as u32);
        }
      }
      
      let index = self.data().path[path_num as usize];
      let qr = map.get_qr_from_index(index as usize);
      let pos = map.tile_position_from_index(index as usize);
      shroom_childs.push(Box::new(Mushroom::new_baby_shroom(self.data().id, pos, self.data().path.clone(), path_num, qr)));
    }
    
    shroom_childs
  }
  
  fn local_update(&mut self, map: &Map, delta_time: f32) {
    if self.data().size.x < MAX_SIZE {
      let old_health = self.data().health as f32;
      let old_size = self.mut_data().size;
      let old_max_health = 1.0 + 10.0*(old_size.x as f32/0.25).ceil();
      self.mut_data().size += self.data().size*self.grow_speed*delta_time;
      if self.mut_data().size.x > 2.5 {
        self.mut_data().sell_price = 10;
        self.mut_data().speed = START_SPEED*0.8*0.8*0.8*0.8;
        if self.last_size_stage < 2.5 {
          self.mut_data().health = self.mut_data().health*2;
        }
      } else if self.mut_data().size.x > 2.0 {
        self.mut_data().sell_price = 8;
        self.mut_data().speed = START_SPEED*0.8*0.8*0.8;
        if self.last_size_stage < 2.0 {
          self.mut_data().health = self.mut_data().health*2;
        }
      } else if self.mut_data().size.x > 1.5 {
        self.mut_data().sell_price = 6;
        self.mut_data().speed = START_SPEED*0.8*0.8;
        if self.last_size_stage < 1.5 {
          self.mut_data().health = self.mut_data().health*2;
        }
      } else if self.mut_data().size.x > 1.0 {
        self.mut_data().sell_price = 4;
        self.mut_data().speed = START_SPEED*0.8;
        if self.last_size_stage < 1.0 {
          self.mut_data().health = self.mut_data().health*2;
        }
      }
      /*
      self.mut_data().health = ((old_health/old_max_health)*(1.0 + 10.0*(self.mut_data().size.x as f32/0.25).ceil()) as f32).ceil() as i32;
      self.mut_data().sell_price = 1 + 5*(self.mut_data().size.x/0.5).floor() as i32;
      self.mut_data().speed *= 1.0-((self.grow_speed)*delta_time);*/
   } else {
     self.mut_data().size = Vector3::new(MAX_SIZE, MAX_SIZE, MAX_SIZE);
   }
    
    self.last_size_stage = self.data().size.x;
  }
}
