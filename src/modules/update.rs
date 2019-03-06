use crate::modules::food::Food;
use crate::modules::appliances::traits::Appliance;
use crate::modules::weapons::Weapon;

use crate::modules::map::Map;

use cgmath::Vector3;

pub fn update_game(map: &Map, appliances: &mut Vec<Box<Appliance>>, foods: &mut Vec<Food>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, delta_time: f32) {
  
  for food in &mut foods.iter_mut() {
    food.update(map, delta_time);
  }
  
  for appliance in &mut appliances.iter_mut() {
    appliance.update(foods, weapons, model_sizes, delta_time);
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
