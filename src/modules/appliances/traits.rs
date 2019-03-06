use maat_graphics::DrawCall;
use crate::modules::food::Food;
use crate::modules::weapons::Weapon;

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3};

pub trait ApplianceClone {
  fn clone_appliance(&self) -> Box<Appliance>;
}

impl<T: 'static + Appliance + Clone> ApplianceClone for T {
  fn clone_appliance(&self) -> Box<Appliance> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Appliance> {
  fn clone(&self) -> Box<Appliance> {
    self.clone_appliance()
  }
}

pub trait Appliance: ApplianceClone {
  fn update(&mut self, foods: &mut Vec<Food>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, delta_time: f32);
  
  fn fire(&mut self);
  
  fn apply_effect(&self);
  fn remove_effects(&self);
  
  fn move_tile(&self);
  fn clean(&self);
  fn upgrade(&mut self);
  
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
