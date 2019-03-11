use crate::modules::food::Food;
use crate::modules::appliances::traits::Appliance;
use crate::modules::weapons::Weapon;
use crate::modules::hexagon::HexagonType;
use crate::modules::map::Map;

use cgmath::Vector3;

pub fn update_game(map: &mut Map, appliances: &mut Vec<Box<Appliance>>, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, mut selected_appliance: &mut Option<usize>, model_sizes: &mut Vec<(String, Vector3<f32>)>, delta_time: f32) {
  
  for food in &mut foods.iter_mut() {
    food.update(map, delta_time);
  }
  
  foods.sort_by(|a,b| (a.get_path_num()).cmp(&b.get_path_num()).reverse());
  
  let mut offset = 0;
  for i in 0..appliances.len() {
    if offset > i {
      break;
    }
    
    appliances[i-offset].update(foods, weapons, model_sizes, map, delta_time);
    if appliances[i-offset].current_life_expectancy() <= 0 {
      let qr = appliances[i-offset].get_qr_location();
      map.set_hexagon_type(qr.x, qr.y, HexagonType::Open);
      appliances.remove(i-offset);
      if let Some(idx) = &mut selected_appliance {
        if i-offset < *idx {
          *idx -= 1;
        }
      }
      offset += 1;
    }
  }
  
  let mut dead_weapons = Vec::new();
  let mut i = 0;
  for weapon in &mut weapons.iter_mut() {
    if weapon.update(delta_time) {
      dead_weapons.push(i);
    }
    
    i += 1;
  }
  
  for i in 0..dead_weapons.len() {
    weapons.remove(dead_weapons[i]-i);
  }
}
