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
  
  for weapon in &mut weapons.iter_mut() {
    weapon.update(delta_time);
  }
}
