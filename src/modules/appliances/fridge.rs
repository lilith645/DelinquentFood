use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::appliances::traits::{Appliance, ApplianceData, Buff, TargetPriority};
use crate::modules::weapons::{Weapon, ColdSnap};
use crate::modules::map::Map;
use crate::modules::hexagon::Hexagon;

use cgmath::{InnerSpace, Deg, Angle, Vector2, Vector3};

#[derive(Clone)]
pub struct Fridge {
  data: ApplianceData,
}

impl Fridge {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, map: &Map) -> Fridge {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    let life_expectancy = 3;
    let range = 1;
    let cost = 85;
    let fire_rate = 1.8;
    let directional_range = false;
    
    Fridge {
      data: ApplianceData::new(tile, size, rotation, "Fridge".to_string(), life_expectancy, range, fire_rate, cost, directional_range, map),
    }
  }
}

impl Appliance for Fridge {
  fn data(&self) -> &ApplianceData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ApplianceData {
    &mut self.data
  }
  
  fn update(&mut self, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, map: &Map, delta_time: f32) -> Vec<(Buff, Vector2<i32>, u32)> {
    self.data.offset.y = 0.0;
    for (reference, size) in model_sizes {
      if *reference == "Hexagon".to_string() {
        self.data.offset.y += size.y;
      }
    }
    
    for food in foods.iter() {
      let location = food.get_tile_location();
      let dist = Hexagon::hex_distance(&Hexagon::new(self.data.tile_location.x, self.data.tile_location.y, "".to_string()), &Hexagon::new(location.x, location.y, "".to_string()));
      
      if dist <= self.get_range() as i32 {
        self.data.rotation.y = self.rotate_towards(self.data.position, food, 90.0);
        
        if self.data.charge >= self.get_fire_rate() {
          let qr = self.get_qr_location();
          let hex = Hexagon::new(qr.x, qr.y, "".to_string());
          
          let radius = self.get_range() as i32;
          let hexagons = Hexagon::generate_hexagon_range(radius, "".to_string());
          
          for hexagon in hexagons {
            let new_hex = Hexagon::hex_add(&hex, &hexagon);
            let mut weapon: Box<Weapon> = Box::new(ColdSnap::new());
            self.add_weapon_modifiers(&mut weapon);
            let pos = map.get_tile_position(new_hex.q(), new_hex.r());
            let position = Vector3::new(pos.x, self.data.position.y, pos.y);
            weapon.launch(position, Vector2::new(new_hex.q(), new_hex.r()), Vector3::new(0.0, 90.0, 0.0), Vector2::new(0.0, 0.0));
            
            weapons.push(weapon);
          }
          
          self.data.charge = 0.0;
        }
        break;
      }
    }
    
    self.data.charge += delta_time;
    
    Vec::new()
  }
  
  fn fire(&mut self) {
    
  }
  
  fn upgrade(&mut self) {
    
  }
  
  fn apply_effect(&self) {
    
  }
  
  fn remove_effects(&self) {
    
  }
  
  fn upgrade_cost(&self) -> i32 {
    1
  }
  
  fn sell(&self) -> i32 {
    1
  }
}
