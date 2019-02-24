use std::ops::{Add, Sub, Mul};

use cgmath::{Vector2, Vector3};

// Orientation information
const F0: f32 = 1.732050808;
const F1: f32 = 1.732050808 * 0.5;
const F2: f32 = 0.0;
const F3: f32 = 3.0 / 2.0;

const B0: f32 = 1.732050808 / 3.0;
const B1: f32 = -1.0 / 3.0;
const B2: f32 = 0.0;
const B3: f32 = 2.0 / 3.0;

pub enum HexDirection {
  NorthEast,
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest
}

pub struct Layout {
  origin: Vector2<f32>,
  size: Vector2<f32>,
}

impl Layout {
  pub fn new(origin: Vector2<f32>, size: Vector2<f32>) -> Layout {
    Layout {
      origin,
      size,
    }
  }
  
  pub fn hex_to_pixel(&self, hexagon: Hexagon) -> Vector2<f32> {
    let x = (F0 * hexagon.q() as f32 + F1 * hexagon.r() as f32) * self.size.x;
    let y = (F2 * hexagon.q() as f32 + F3 * hexagon.r() as f32) * self.size.y;
    
    Vector2::new(x + self.origin.x, y + self.origin.y)
  }
  
  pub fn hex_corner_offset(&self, corner: i32) -> Vector2<f32> {
    let size = self.size;
    let angle = 2.0 * 3.14 * (0.5 + corner as f32) / 6.0;
    
    Vector2::new(size.x * (angle).cos(), size.y * (angle).sin())
  }
  
  pub fn polygon_corners(&self, hexagon: Hexagon) -> Vec<Vector2<f32>> {
    let mut corners: Vec<Vector2<f32>> = Vec::new();
    let center = self.hex_to_pixel(hexagon);
    for i in 0..6 {
      let offset = self.hex_corner_offset(i);
      corners.push(Vector2::new(center.x + offset.x, center.y + offset.y));
    }
    
    corners
  }
}

#[derive(Clone)]
pub struct Hexagon {
  position: Vector3<i32>,
  model: String,
}

impl Hexagon {
  pub fn new(q: i32, r: i32, model: String) -> Hexagon {
    Hexagon {
      position: Vector3::new(q, r, -q-r),
      model,
    }
  }
  
  pub fn plain(&mut self) {
    self.model = "Hexagon".to_string();
  }
  
  pub fn highlight(&mut self) {
    self.model = "BlueHexagon".to_string();
  }
  
  pub fn set_as_path(&mut self) {
    self.model = "GreenHexagon".to_string();
  }
  
  pub fn get_model(&self) -> String {
    self.model.to_string()
  }
  
  pub fn q(&self) -> i32 {
    self.position.x
  }
  
  pub fn r(&self) -> i32 {
    self.position.y
  }
  
  pub fn s(&self) -> i32 {
    self.position.z
  }
  
  pub fn length(&self) -> i32 {
    (((self.position.x).abs() + (self.position.y).abs() + (self.position.z).abs()) as f32 * 0.5) as i32
  }
  
  pub fn hex_distance(hexagon: Hexagon, other_hexagon: Hexagon) -> i32 {
    Hexagon::hex_sub(hexagon, other_hexagon).length()
  }
  
  pub fn hex_direction(direction: HexDirection) -> Hexagon {
    match direction {
      HexDirection::NorthEast => {
        Hexagon::new(1, -1, "hexagon".to_string())
      },
      HexDirection::East => {
        Hexagon::new(1, 0, "hexagon".to_string())
      },
      HexDirection::SouthEast => {
        Hexagon::new(0, 1, "hexagon".to_string())
      },
      HexDirection::SouthWest => {
        Hexagon::new(-1, 1, "hexagon".to_string())
      },
      HexDirection::West => {
        Hexagon::new(-1, 0, "hexagon".to_string())
      },
      HexDirection::NorthWest => {
        Hexagon::new(0, -1, "hexagon".to_string())
      },
    }
  }
  
  pub fn hex_neigbor(hexagon: Hexagon, direction: HexDirection) -> Hexagon {
    Hexagon::hex_add(hexagon, Hexagon::hex_direction(direction))
  }
  
  pub fn hex_add(hexagon: Hexagon, other_hexagon: Hexagon) -> Hexagon {
    Hexagon::new(hexagon.q() + other_hexagon.q(),
                 hexagon.r() + other_hexagon.r(), 
                 hexagon.get_model())
  }
  
  pub fn hex_sub(hexagon: Hexagon, other_hexagon: Hexagon) -> Hexagon {
    Hexagon::new(hexagon.q() - other_hexagon.q(),
                 hexagon.r() - other_hexagon.r(), 
                 hexagon.get_model())
  }
  
  pub fn hex_mul(hexagon: Hexagon, other_hexagon: Hexagon) -> Hexagon {
    Hexagon::new(hexagon.q() * other_hexagon.q(),
                 hexagon.r() * other_hexagon.r(), 
                 hexagon.get_model())
  }
  
  pub fn hex_equals(hexagon: Hexagon, other_hexagon: Hexagon) -> bool {
    (hexagon.q() == other_hexagon.q() && hexagon.r() == other_hexagon.r())
  }
}

