use maat_graphics::DrawCall;
use crate::modules::food::Food;

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3};

pub trait TowerClone {
  fn clone_tower(&self) -> Box<Tower>;
}

impl<T: 'static + Tower + Clone> TowerClone for T {
  fn clone_tower(&self) -> Box<Tower> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Tower> {
  fn clone(&self) -> Box<Tower> {
    self.clone_tower()
  }
}

pub trait Tower: TowerClone {
  fn update(&mut self, foods: &mut Vec<Food>, model_sizes: &mut Vec<(String, Vector3<f32>)>, delta_time: f32);
  
  fn fire(&mut self);
  
  fn upgrade(&mut self);
  
  fn apply_effect(&self);
  fn remove_effects(&self);
  
  fn upgrade_cost(&self) -> u32;
  fn sell(&self) -> u32;
  
  fn rotate_towards(&self, position: Vector3<f32>, food: &Food, angle_offset: f32) -> f32 {
    let loc = food.get_location();
    let direction = Vector2::new(loc.x-position.x, loc.y-position.z).normalize();
    let mut angle = Deg::atan2(direction.x, direction.y);
    
    angle.0 as f32+angle_offset
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>);
}



/*
impl Tower {
  
}*/
