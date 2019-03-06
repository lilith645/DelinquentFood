use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::appliances::traits::Appliance;
use crate::modules::weapons::{Weapon, Dish};
use crate::modules::map::Map;
use crate::modules::hexagon::Hexagon;

use cgmath::{InnerSpace, Deg, Angle, Vector2, Vector3};

#[derive(Clone)]
pub struct Dishwasher {
  position: Vector3<f32>,
  offset: Vector3<f32>,
  size: Vector3<f32>,
  rotation: Vector3<f32>,
  model: String,
  tile_location: Vector2<i32>,
  range: u32,
  charge: f32,
  fire_rate: f32,
}

impl Dishwasher {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, map: &Map) -> Dishwasher {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    
    Dishwasher {
      position: Vector3::new(position.x, 0.0, position.y),
      offset: Vector3::new(0.0, 0.0, 0.0),
      size,
      rotation,
      model: "Dishwasher".to_string(),
      tile_location: tile,
      range: 3,
      charge: 0.0,
      fire_rate: 1.0,
    }
  }
}


impl Appliance for Dishwasher {
  fn update(&mut self, foods: &mut Vec<Food>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, delta_time: f32) {
    self.offset.y = 0.0;
    for (reference, size) in model_sizes {
      if *reference == "Hexagon".to_string() {
        self.offset.y += size.y;
      }
      if *reference == "Dishwasher".to_string() {
        self.offset.y += size.y*0.5;
      }
    }
    
    for food in foods {
      let location = food.get_tile_location();
      let dist = Hexagon::hex_distance(Hexagon::new(self.tile_location.x, self.tile_location.y, "".to_string()), Hexagon::new(location.x, location.y, "".to_string()));
      
      if dist <= self.range as i32 {
        self.rotation.y = self.rotate_towards(self.position, food, 90.0);
        
        if self.charge >= self.fire_rate {
          let loc = food.get_location();
          let direction = Vector2::new(loc.x-self.position.x, loc.y-self.position.z).normalize();
          
          let mut weapon = Dish::new();
          weapon.launch(self.position+self.offset, self.tile_location, self.rotation, direction);
          
          weapons.push(Box::new(weapon));
          
          self.charge = 0.0;
        }
      }
    }
    
    self.charge += delta_time;
  }
  
  fn fire(&mut self) {
    
  }
  
  fn move_tile(&self) {
    
  }
  
  fn clean(&self) {
    
  }
  
  fn upgrade(&mut self) {
    
  }
  
  fn apply_effect(&self) {
    
  }
  
  fn remove_effects(&self) {
    
  }
  
  fn upgrade_cost(&self) -> u32 {
    1
  }
  
  fn sell(&self) -> u32 {
    1
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position+self.offset, self.size, self.rotation, self.model.to_string()));
  }
}
