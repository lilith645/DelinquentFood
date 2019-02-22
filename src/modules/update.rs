use crate::modules::food::Food;
use crate::modules::towers::traits::Tower;

use crate::modules::maploader::Map;

pub fn update_game(map: &Map, towers: &mut Vec<Box<Tower>>, foods: &mut Vec<Food>, delta_time: f32) {
  for food in &mut foods.iter_mut() {
    food.update(map, delta_time);
  }
  
  for tower in &mut towers.iter_mut() {
    tower.update(delta_time);
  }
}
