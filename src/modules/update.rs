use crate::modules::food::Food;
use crate::modules::towers::traits::Tower;

use crate::modules::map::Map;

use cgmath::Vector3;

pub fn update_game(map: &Map, towers: &mut Vec<Box<Tower>>, foods: &mut Vec<Food>, model_sizes: &mut Vec<(String, Vector3<f32>)>, delta_time: f32) {
  for food in &mut foods.iter_mut() {
    food.update(map, delta_time);
  }
  
  for tower in &mut towers.iter_mut() {
    tower.update(foods, model_sizes, delta_time);
  }
}
