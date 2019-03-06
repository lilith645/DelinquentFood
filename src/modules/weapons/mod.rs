pub use self::dish::Dish;

mod dish;

use maat_graphics::DrawCall;

use crate::modules::food::Food;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub enum Debuff {
  Slow,
  Freeze,
  Reverse,
}

#[derive(Clone)]
enum WeaponType {
  Tile,
  Projectile,
  AntiFood,
}

#[derive(Clone)]
struct WeaponData {
  position: Vector3<f32>,
  tile_position: Vector2<i32>,
  rotation: Vector3<f32>,
  size: Vector3<f32>,
  direction: Vector2<f32>,
  velocity: f32,
  damage: f32,
  debuffs: Vec<Debuff>,
  weapon_type: WeaponType,
  range: i32, // 0 is not range limit
  model: String,
}

impl WeaponData {
  pub fn new(vel: f32, dmg: f32, rng: i32, sz: Vector3<f32>, w_type: WeaponType, model: String) -> WeaponData {
    WeaponData {
      position: Vector3::new(0.0, 0.0, 0.0),
      tile_position: Vector2::new(0,0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      size: sz,
      direction: Vector2::new(0.0, 0.0),
      velocity: vel,
      damage: dmg,
      debuffs: Vec::new(),
      weapon_type: w_type,
      range: rng, // 0 is not range limit
      model,
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
  
  fn launch(&mut self, position: Vector3<f32>, tile_position: Vector2<i32>, rotation: Vector3<f32>, direction: Vector2<f32>) {
    match self.data().weapon_type {
      WeaponType::Projectile => {
        self.mut_data().position = position;
        self.mut_data().rotation = rotation;
        self.mut_data().direction = direction;
      },
      WeaponType::Tile => {
        self.mut_data().tile_position = tile_position;
      }
      WeaponType::AntiFood => {
        
      },
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    match self.data().weapon_type {
      WeaponType::Projectile => {
        self.mut_data().position.x += self.data().velocity*self.data().direction.x*delta_time;
        self.mut_data().position.z += self.data().velocity*self.data().direction.y*delta_time;
      },
      WeaponType::Tile => {
        
      }
      WeaponType::AntiFood => {
        
      },
    }
  }
  
  fn collision(&mut self);
  
  fn hit_target(&self, food: &mut Food);
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let position = self.data().position;
    let rotation = self.data().rotation;
    let size = self.data().size;
    let model = self.data().model.to_string();
    draw_calls.push(DrawCall::draw_model(position, size, rotation, model));
  }
}
