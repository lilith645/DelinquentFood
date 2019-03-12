pub use self::dish::Dish;
pub use self::coldsnap::ColdSnap;
pub use self::tenderizer::Tenderizer;

mod dish;
mod coldsnap;
mod tenderizer;

use maat_graphics::DrawCall;

use crate::modules::food::Food;
use crate::modules::hexagon::Layout;
use crate::modules::hexagon::Hexagon;
use crate::modules::map::Map;

use cgmath::{Vector2, Vector3};

#[derive(Clone, PartialEq)]
pub enum Debuff {
  Slow(f32),
  Freeze(f32),
  Reverse(f32),
}

#[derive(Clone)]
pub enum WeaponType {
  Tile,
  Projectile,
  AntiFood,
}

#[derive(Clone)]
pub struct WeaponData {
  position: Vector3<f32>,
  tile_position: Vector2<i32>,
  rotation: Vector3<f32>,
  size: Vector3<f32>,
  direction: Vector2<f32>,
  velocity: f32,
  rotation_velocity: Vector3<f32>,
  damage: i32,
  pierce: i32,
  debuffs: Vec<Debuff>,
  weapon_type: WeaponType,
  model: String,
  food_hit: Vec<i32>,
  timer: f32,
}

impl WeaponData {
  pub fn new(vel: f32, rot_vel: Vector3<f32>, dmg: i32, prc: i32, timer: f32, sz: Vector3<f32>, w_type: WeaponType, debuffs: Vec<Debuff>, model: String) -> WeaponData {
    WeaponData {
      position: Vector3::new(0.0, 0.0, 0.0),
      tile_position: Vector2::new(0,0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      size: sz,
      direction: Vector2::new(0.0, 0.0),
      velocity: vel,
      rotation_velocity: rot_vel,
      damage: dmg,
      pierce: prc,
      debuffs,
      weapon_type: w_type,
      model,
      food_hit: Vec::new(),
      timer: timer,
    }
  }
}

pub trait WeaponClone {
  fn clone_weapon(&self) -> Box<Weapon>;
}

impl<T: 'static + Weapon + Clone> WeaponClone for T {
  fn clone_weapon(&self) -> Box<Weapon> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Weapon> {
  fn clone(&self) -> Box<Weapon> {
    self.clone_weapon()
  }
}

pub trait Weapon: WeaponClone {
  fn data(&self) -> &WeaponData;
  fn mut_data(&mut self) -> &mut WeaponData;
  
  fn add_pierce(&mut self, extra_pierce: i32) {
    self.mut_data().pierce += extra_pierce;
  }
  
  fn damage_multiplier(&mut self, percentage: f32) {
    self.mut_data().damage += (self.data().damage as f32*percentage).ceil() as i32;
  }
  
  fn get_hexagon(&self, map: &Map) -> Hexagon {
    map.pixel_to_hex(self.data().position.x, self.data().position.z)
  }
  
  fn launch(&mut self, position: Vector3<f32>, tile_position: Vector2<i32>, rotation: Vector3<f32>, direction: Vector2<f32>) {
    match self.data().weapon_type {
      WeaponType::Projectile => {
        self.mut_data().position = position;
        self.mut_data().rotation = rotation;
        self.mut_data().direction = direction;
      },
      WeaponType::Tile => {
        self.mut_data().position = position;
        self.mut_data().tile_position = tile_position;
        self.mut_data().rotation = rotation;
      }
      WeaponType::AntiFood => {
        
      },
    }
  }
  
  fn update(&mut self, delta_time: f32) -> bool {
    match self.data().weapon_type {
      WeaponType::Projectile => {
        self.mut_data().position.x += self.data().velocity*self.data().direction.x*delta_time;
        self.mut_data().position.z += self.data().velocity*self.data().direction.y*delta_time;
        self.mut_data().rotation.x += self.data().rotation_velocity.x*delta_time;
        self.mut_data().rotation.y += self.data().rotation_velocity.y*delta_time;
        self.mut_data().rotation.z += self.data().rotation_velocity.z*delta_time;
      },
      WeaponType::Tile => {
        self.mut_data().timer -= delta_time;
        if self.mut_data().timer <= 0.0 {
          self.mut_data().pierce = 0;
        }
      }
      WeaponType::AntiFood => {
        
      },
    }
    
    self.data().position.x > 250.0 || self.data().position.x < -250.0 || self.data().position.z > 250.0 || self.data().position.z < -250.0 || self.is_broken()
  }
  
  fn is_broken(&self) -> bool {
    self.data().pierce <= 0
  }
  
  fn hasnt_hit(&self, id: i32) -> bool {
    !self.data().food_hit.contains(&id)
  }
  
  fn hit_target(&mut self, food: &mut Box<Food>);
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let position = self.data().position;
    let rotation = self.data().rotation;
    let size = self.data().size;
    let model = self.data().model.to_string();
    draw_calls.push(DrawCall::draw_model(position, size, rotation, model));
  }
}
