
use crate::modules::weapons::{Weapon, WeaponData, WeaponType};
use crate::modules::food::Food;

use cgmath::Vector3;

#[derive(Clone)]
pub struct ColdSnap {
  data: WeaponData,
}

impl ColdSnap {
  pub fn new() -> ColdSnap {
    ColdSnap {
      data: WeaponData::new(0.0, 1, 1, 0.6, Vector3::new(1.9, 0.6, 1.9), WeaponType::Tile, "BlueHexagon".to_string()),
    }
  }
}

impl Weapon for ColdSnap {
  fn data(&self) -> &WeaponData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut WeaponData {
    &mut self.data
  }
  
  fn hit_target(&mut self, food: &mut Food) {
    food.apply_damage(self.data.damage);
    self.data.food_hit.push(food.get_id());
  }
}
