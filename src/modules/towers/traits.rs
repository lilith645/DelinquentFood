use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector3};

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
  fn update(&mut self, delta_time: f32);
  
  fn fire(&mut self);
  
  fn upgrade(&mut self);
  
  fn apply_effect(&self);
  fn remove_effects(&self);
  
  fn upgrade_cost(&self) -> u32;
  fn sell(&self) -> u32;
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>);
}



/*
impl Tower {
  
}*/
