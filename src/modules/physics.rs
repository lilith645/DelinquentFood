use crate::modules::map::Map;
use crate::modules::food::Food;
use crate::modules::weapons::Weapon;
use crate::modules::hexagon::Hexagon;

use cgmath::{Vector3};

pub fn collisions(map: &Map, foods: &mut Vec<Food>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, _delta_time: f32) {
  for food in &mut foods.iter_mut() {
    let food_tile = food.get_tile_location();
    let hex = Hexagon::new(food_tile.x, food_tile.y, "".to_string());
    
    for weapon in &mut weapons.iter_mut() {
      if weapon.hasnt_hit(food.get_id()) {
        let w_hex = weapon.get_hexagon(map);
        
        if Hexagon::hex_equals(hex.clone(), w_hex) {
          weapon.hit_target(food);
        }
      }
    }
    
    let mut offset = 0;
    for i in 0..weapons.len() {
      if offset > i {
        break;
      }
      
      if weapons[i-offset].is_broken() {
        weapons.remove(i-offset);
        offset += 1;
      }
    }
  }
  
  let mut offset = 0;
  for i in 0..foods.len() {
    if offset > i {
      break;
    }
    
    if foods[i-offset].is_cooked() {
      foods.remove(i-offset);
      offset += 1;
    }
  }
}
