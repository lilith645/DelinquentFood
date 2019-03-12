use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::appliances::traits::{Appliance, ApplianceData, Buff, TargetPriority};
use crate::modules::weapons::{Weapon, Tenderizer};
use crate::modules::map::Map;
use crate::modules::hexagon::Hexagon;

use cgmath::{InnerSpace, Deg, Angle, Vector2, Vector3};

#[derive(Clone)]
pub struct MeatTenderizer {
  data: ApplianceData,
}

impl MeatTenderizer {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, map: &Map) -> MeatTenderizer {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    let life_expectancy = 4;
    let range = 1;
    let cost = 75;
    let fire_rate = 2.2;
    let directional_range = true;
    
    MeatTenderizer {
      data: ApplianceData::new(tile, size, rotation, "MeatTenderizer".to_string(), life_expectancy, range, fire_rate, cost, directional_range, map),
    }
  }
}

impl Appliance for MeatTenderizer {
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
    
    let some_food = self.get_prioritised_food(foods);
    if let Some(food) = some_food {
      self.data.rotation.y = self.rotate_towards(self.data.position, &food, 90.0);
      
      if self.data.charge >= self.data.fire_rate {
        let loc = food.get_location();
        let location = food.get_tile_location();
        let direction = Vector2::new(loc.x-self.data.position.x, loc.y-self.data.position.z).normalize();
        
        let mut weapon = Tenderizer::new();
        let pos = map.get_tile_position(location.x, location.y);
        let position = Vector3::new(pos.x, self.data.position.y, pos.y);
        weapon.launch(position, Vector2::new(location.x, location.y), Vector3::new(0.0, 90.0, 0.0), Vector2::new(0.0, 0.0));
        
        weapons.push(Box::new(weapon));
        
        self.data.charge = 0.0;
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
