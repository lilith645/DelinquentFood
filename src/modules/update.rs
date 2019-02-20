use crate::modules::food::Food;

use crate::modules::maploader::Map;

pub fn update_game(map: &Map, foods: &mut Vec<Food>, delta_time: f32) {
  for food in &mut foods.iter_mut() {
    food.update(map, delta_time);
  }
}
