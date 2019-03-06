
use crate::modules::weapons::{Weapon, WeaponData, WeaponType};
use crate::modules::food::Food;

use cgmath::Vector3;

#[derive(Clone)]
pub struct Dish {
  data: WeaponData,
}

impl Dish {
  pub fn new() -> Dish {
    Dish {
      data: WeaponData::new(80.0, 1, 2, 0, Vector3::new(0.5, 0.5, 0.5), WeaponType::Projectile, "Spoon".to_string()),
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
  
  fn hit_target(&mut self, food: &mut Food) {
    food.apply_damage(self.data.damage);
    self.data.pierce -= 1;
    self.data.food_hit.push(food.get_id());
  }
}
