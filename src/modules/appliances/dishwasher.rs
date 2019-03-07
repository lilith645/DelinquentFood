use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::appliances::traits::{Appliance, ApplianceData, TargetPriority};
use crate::modules::weapons::{Weapon, Dish};
use crate::modules::map::Map;
use crate::modules::hexagon::Hexagon;

use cgmath::{InnerSpace, Deg, Angle, Vector2, Vector3};

#[derive(Clone)]
pub struct Dishwasher {
  data: ApplianceData,
}

impl Dishwasher {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, map: &Map) -> Dishwasher {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    let range = 3;
    let charge = 0.0;
    let fire_rate = 1.0;
    
    Dishwasher {
      data: ApplianceData::new(tile, size, rotation, "Dishwasher".to_string(), range, fire_rate, charge, map),
    }
  }
}

impl Appliance for Dishwasher {
  fn data(&self) -> &ApplianceData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ApplianceData {
    &mut self.data
  }
  
  fn update(&mut self, foods: &mut Vec<Food>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, _map: &Map, delta_time: f32) {
    self.data.offset.y = 0.0;
    for (reference, size) in model_sizes {
      if *reference == "Hexagon".to_string() {
        self.data.offset.y += size.y;
      }
      if *reference == "Dishwasher".to_string() {
        self.data.offset.y += size.y*0.5;
      }
    }
    
    for food in foods.iter() {
      let location = food.get_tile_location();
      let dist = Hexagon::hex_distance(Hexagon::new(self.data.tile_location.x, self.data.tile_location.y, "".to_string()), Hexagon::new(location.x, location.y, "".to_string()));
      
      if dist <= self.data.range as i32 {
        self.data.rotation.y = self.rotate_towards(self.data.position, food, 90.0);
        
        if self.data.charge >= self.data.fire_rate {
          let loc = food.get_location();
          let direction = Vector2::new(loc.x-self.data.position.x, loc.y-self.data.position.z).normalize();
          
          let mut weapon = Dish::new();
          weapon.launch(self.data.position+self.data.offset, self.data.tile_location, self.data.rotation, direction);
          
          weapons.push(Box::new(weapon));
          
          self.data.charge = 0.0;
        }
        break;
      }
    }
    
    self.data.charge += delta_time;
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
}
