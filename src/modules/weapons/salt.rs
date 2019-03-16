
use crate::modules::weapons::{Weapon, WeaponData, WeaponType};
use crate::modules::food::Food;

use cgmath::Vector3;

#[derive(Clone)]
pub struct Salt {
  data: WeaponData,
}

impl Salt {
  pub fn new() -> Salt {
    let velocity = 160.0;
    let rotation_velocity = Vector3::new(360.0, 360.0, 360.0);
    let damage = 1;
    let pierce = 1;
    let timer = 0.0;
    let scale = Vector3::new(0.4, 0.4, 0.4);
    let debuff = Vec::new();
    Salt {
      data: WeaponData::new(velocity, rotation_velocity, damage, pierce, timer, scale, WeaponType::Projectile, debuff, "Salt".to_string()),
    }
  }
}

impl Weapon for Salt {
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
