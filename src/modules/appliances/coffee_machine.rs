use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::appliances::traits::{Appliance, ApplianceData, Buff, TargetPriority};
use crate::modules::weapons::{Weapon, Dish};
use crate::modules::map::Map;
use crate::modules::hexagon::Hexagon;

use cgmath::{InnerSpace, Deg, Angle, Vector2, Vector3};

#[derive(Clone)]
pub struct CoffeeMachine {
  data: ApplianceData,
}

impl CoffeeMachine {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, map: &Map) -> CoffeeMachine {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    let life_expectancy = 5;
    let range = 2;
    let cost = 120;
    let buffing_rate = 0.8;
    let directional_range = false;
    
    CoffeeMachine {
      data: ApplianceData::new(tile, size, rotation, "CoffeeMachine".to_string(), life_expectancy, range, buffing_rate, cost, directional_range, map),
    }
  }
}

impl Appliance for CoffeeMachine {
  fn data(&self) -> &ApplianceData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ApplianceData {
    &mut self.data
  }
  
  fn update(&mut self, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, _map: &Map, delta_time: f32) -> Vec<(Buff, Vector2<i32>, u32)> {
    self.data.offset.y = 0.0;
    for (reference, size) in model_sizes {
      if *reference == "Hexagon".to_string() {
        self.data.offset.y += size.y;
      }
    }
    
    self.data.charge += delta_time;
    
    vec!((Buff::AttackSpeed, self.get_qr_location(), self.get_range()),
         (Buff::LifeExpectancy, self.get_qr_location(), self.get_range()),
         (Buff::SellPrice, self.get_qr_location(), self.get_range()),
         (Buff::Range, self.get_qr_location(), self.get_range()))
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
