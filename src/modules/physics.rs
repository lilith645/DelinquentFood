use crate::modules::map::Map;
use crate::modules::food::Food;
use crate::modules::weapons::Weapon;
use crate::modules::hexagon::Hexagon;

use cgmath::{Vector3};

pub fn collisions(map: &Map, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, bin: &mut i32, money: &mut i32, _delta_time: f32) {
  for food in &mut foods.iter_mut() {
    //let food_tile = food.get_tile_location();
    //let hex = Hexagon::new(food_tile.x, food_tile.y, "".to_string());
    let food_pos = food.get_location();
    let hex = map.pixel_to_hex(food_pos);
    
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
  
  let mut food_children = Vec::new();
  
  let mut offset = 0;
  for i in 0..foods.len() {
    if offset > i {
      break;
    }
    
    if foods[i-offset].is_cooked() || foods[i-offset].is_rotten() {
      if foods[i-offset].is_rotten() {
        // Do something
        *bin += 10;
      } else {
        *money += foods[i-offset].sell_price();
      }
      
      food_children.append(&mut foods[i-offset].get_children());
      foods.remove(i-offset);
      offset += 1;
    }
  }
  
  foods.append(&mut food_children);
}
