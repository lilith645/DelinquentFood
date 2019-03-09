
use crate::modules::weapons::{Weapon, WeaponData, WeaponType};
use crate::modules::food::Food;

use cgmath::Vector3;

#[derive(Clone)]
pub struct Dish {
  data: WeaponData,
}

impl Dish {
  pub fn new() -> Dish {
    let velocity = 80.0;
    let rotation_velocity = Vector3::new(360.0, 360.0, 360.0);
    let damage = 3;
    let pierce = 2;
    let timer = 0.0;
    let scale = Vector3::new(1.0, 1.0, 1.0);
    let debuff = Vec::new();
    Dish {
      data: WeaponData::new(velocity, rotation_velocity, damage, pierce, timer, scale, WeaponType::Projectile, debuff, "Plate".to_string()),
    }
  }
}

impl Weapon for Dish {
  fn data(&self) -> &WeaponData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut WeaponData {
    &mut self.data
  }
  
  fn hit_target(&mut self, food: &mut Box<Food>) {
    food.apply_damage(self.data.damage);
    self.data.pierce -= 1;
    self.data.food_hit.push(food.get_id());
  }
}
