use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::appliances::traits::{Appliance, ApplianceData, Buff, TargetPriority};
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
    let life_expectancy = 2;
    let range = 3;
    let cost = 65;
    let fire_rate = 1.2;
    let directional_range = false;
    
    Dishwasher {
      data: ApplianceData::new(tile, size, rotation, "Dishwasher".to_string(), life_expectancy, range, fire_rate, cost, directional_range, map),
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
  
  fn update(&mut self, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, map: &Map, delta_time: f32) -> Vec<(Buff, Vector2<i32>, u32)> {
    self.data.offset.y = 0.0;
    for (reference, size) in model_sizes {
      if *reference == "Hexagon".to_string() {
        self.data.offset.y += size.y;
      }
      if *reference == "Dishwasher".to_string() {
        self.data.offset.y += size.y*0.5;
      }
    }
    
    let some_food = self.get_prioritised_food(foods, map);
    if let Some(food) = some_food {
      self.data.rotation.y = self.rotate_towards(self.data.position, &food, 90.0);
      
      if self.data.charge >= self.get_fire_rate() {
        let loc = food.get_location();
        let direction = Vector2::new(loc.x-self.data.position.x, loc.y-self.data.position.z).normalize();
        
        let mut weapon: Box<Weapon> = Box::new(Dish::new());
        self.add_weapon_modifiers(&mut weapon);
        weapon.launch(self.data.position+self.data.offset, self.data.tile_location, self.data.rotation, direction);
        
        weapons.push(weapon);
        
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
