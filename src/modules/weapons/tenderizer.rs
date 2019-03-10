
use crate::modules::weapons::{Weapon, WeaponData, WeaponType, Debuff};
use crate::modules::food::Food;

use cgmath::Vector3;

#[derive(Clone)]
pub struct Tenderizer {
  data: WeaponData,
}

impl Tenderizer {
  pub fn new() -> Tenderizer {
    let velocity = 0.0;
    let rotation_velocity = Vector3::new(0.0, 0.0, 0.0);
    let damage = 5;
    let pierce = 10;
    let timer = 0.6;
    let scale = Vector3::new(1.9, 0.3, 1.9);
    let debuff = vec!();
    Tenderizer {
      data: WeaponData::new(velocity, rotation_velocity, damage, pierce, timer, scale, WeaponType::Tile, debuff, "RedHexagon".to_string()),
    }
  }
}

impl Weapon for Tenderizer {
  fn data(&self) -> &WeaponData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut WeaponData {
    &mut self.data
  }
  
  fn hit_target(&mut self, food: &mut Box<Food>) {
    food.apply_damage(self.data.damage);
    food.apply_debuffs(self.data.debuffs.clone());
    self.data.food_hit.push(food.get_id());
  }
}
