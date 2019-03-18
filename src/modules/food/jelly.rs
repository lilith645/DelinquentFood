use maat_graphics::math;

use std::f32::consts::FRAC_PI_2;

use crate::modules::food::{Food, FoodData};
use crate::modules::map::Map;

use cgmath::{InnerSpace, Vector2, Vector3};

const MAX_HEALTH: i32 = 250;

#[derive(Clone)]
pub struct Jelly {
  data: FoodData,
}

impl Jelly {
  pub fn new(id: i32, position: Vector2<f32>, path: Vec<u32>, location: Vector2<i32>) -> Jelly {
    let health = MAX_HEALTH;
    let speed = 12.0;
    let position = Vector3::new(position.x, 0.0, position.y);
    let rotation = Vector3::new(0.0, -30.0, 0.0);
    let size = Vector3::new(3.0, 3.0, 3.0);
    let sell_price = 50;
    let mut path = path.clone();
    for i in 1..(path.len() as f32*0.5).floor() as usize {
      path.remove(i*2-i);
    }
    
    Jelly {
      data: FoodData::new(id, position, rotation, size, speed, health, "Salt".to_string(), path, location, sell_price),
    }
  }
}

impl Food for Jelly {
  fn data(&self) -> &FoodData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut FoodData {
    &mut self.data
  }
  
  fn get_bin_space(&self) -> i32 {
    (30.0 * (self.data().health as f32 / MAX_HEALTH as f32)).ceil() as i32
  }
  
  fn get_children(&self, map: &Map) -> Vec<Box<Food>> {
    Vec::new()
  }
  
  fn local_update(&mut self, map: &Map, move_angle: f32,  delta_time: f32) {
    let diff = self.data.rotation.y-move_angle;
    
    if diff.abs() > 5.0 {
      self.data.rotation.y += 360.0*-(diff.signum())*delta_time;
    }
    
    let path_location = map.tile_position_from_index(self.data().path[self.data().path_number as usize-1] as usize);
    
    let dist = (path_location - self.data().target).magnitude();
    let half_dist = dist*0.5;
    
    let mut relative_pos = (self.data.position.xz() - path_location).magnitude();
    
    if relative_pos > half_dist {
      relative_pos = dist-relative_pos; 
    }
    
    let smoothed_ratio_sin = ((relative_pos/half_dist)*FRAC_PI_2).sin();
    let smoothed_ratio_ln = ((relative_pos/half_dist)).ln()*0.1;
    
    let height = 20.0*smoothed_ratio_sin;
    self.data.size.y = 6.0*smoothed_ratio_ln + 2.0;
    self.data.size.x = 2.0-7.0*smoothed_ratio_ln;
    self.data.size.z = self.data.size.x;
    self.data.position.y = height;
    
    //self.data.position.y = 1.0 + 2.0*self.data().total_dt.sin();
  }
}
