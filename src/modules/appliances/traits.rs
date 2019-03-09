use maat_graphics::DrawCall;
use crate::modules::food::Food;
use crate::modules::weapons::Weapon;
use crate::modules::map::Map;
use crate::modules::hexagon::{Layout, Hexagon};

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3};

#[derive(Clone)]
pub enum TargetPriority {
  First,
  Last,
  Close,
  //Far,
}

#[derive(Clone)]
pub struct ApplianceData {
  pub position: Vector3<f32>,
  pub offset: Vector3<f32>,
  pub size: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub model: String,
  pub tile_location: Vector2<i32>,
  pub range: u32,
  pub charge: f32,
  pub fire_rate: f32,
  pub target: TargetPriority,
  pub life_expectancy: i32,
  pub draw_range: bool,
}

impl ApplianceData {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, model: String, rng: u32, fire_rate: f32, charge: f32, map: &Map) -> ApplianceData {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    
    ApplianceData {
      position: Vector3::new(position.x, 0.0, position.y),
      offset: Vector3::new(0.0, 0.0, 0.0),
      size,
      rotation,
      model,
      tile_location: tile,
      range: rng,
      charge,
      fire_rate,
      target: TargetPriority::First,
      life_expectancy: 1,
      draw_range: false,
    }
  }
}

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
  fn data(&self) -> &ApplianceData;
  fn mut_data(&mut self) -> &mut ApplianceData;
  
  fn update(&mut self, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, map: &Map, delta_time: f32);
  
  fn fire(&mut self);
  
  fn apply_effect(&self);
  fn remove_effects(&self);
  
  fn move_tile(&self);
  fn clean(&self);
  fn upgrade(&mut self);
  
  fn upgrade_cost(&self) -> u32;
  fn sell(&self) -> u32;
  
  fn get_position(&self) -> Vector3<f32> {
    self.data().position
  }
  
  fn get_range(&self) -> u32 {
    self.data().range
  }
  
  fn get_qr_locaiton(&self) -> Vector2<i32> {
    self.data().tile_location
  }
  
  fn rotate_towards(&self, position: Vector3<f32>, food: &Box<Food>, angle_offset: f32) -> f32 {
    let loc = food.get_location();
    let direction = Vector2::new(loc.x-position.x, loc.y-position.z).normalize();
    let mut angle = Deg::atan2(direction.x, direction.y);
    
    angle.0 as f32+angle_offset
  }
  
  fn set_qr_location(&mut self, q: i32, r: i32, map: &Map) {
    let pos = map.get_tile_position(q, r);
    self.mut_data().tile_location = Vector2::new(q,r);
    self.mut_data().position.x = pos.x;
    self.mut_data().position.z = pos.y;
  }
  
  fn should_draw_range(&mut self, should_draw: bool) {
    self.mut_data().draw_range = should_draw;
  }
  
  fn draw_range(&self, map: &Map, draw_calls: &mut Vec<DrawCall>) {
    let mut layout = map.get_layout();
    let new_origin = Vector2::new(self.data().position.x, self.data().position.z);
    layout.set_origin(new_origin);
    
    // draw hexagons
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
    let radius = self.data().range as i32;
    for q in -radius..radius+1 {
      let r1 = (-radius).max(-q - radius);
      let r2 = radius.min(-q + radius);
      
      for r in r1..r2+1 {
        let dist = Hexagon::hex_distance(Hexagon::new(0, 0, "".to_string()), Hexagon::new(q, r, "".to_string()))%4;
        let mut texture = "PurpleHexagon".to_string();
        
        hexagons.push(Hexagon::new(q, r, texture.to_string()));
      }
    }
    
    for hexagon in hexagons {
      let height = 1.2;
      hexagon.draw_hologram(map, &layout, height, draw_calls);
    }
  }
  
  fn draw_hologram(&self, map: &Map, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_hologram_model(self.data().position+self.data().offset, self.data().size, self.data().rotation, self.data().model.to_string()));
    if self.data().draw_range {
      self.draw_range(map, draw_calls);
    }
  }
  
  fn draw(&self, map: &Map, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.data().position+self.data().offset, self.data().size, self.data().rotation, self.data().model.to_string()));
    if self.data().draw_range {
      self.draw_range(map, draw_calls);
    }
  }
}



/*
impl Tower {
  
}*/
