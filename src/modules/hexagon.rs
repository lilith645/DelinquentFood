use maat_graphics::DrawCall;

use crate::modules::map::Map;

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

#[derive(Clone, PartialEq)]
pub enum HexagonType {
  Start,
  End,
  Path,
  Open,
  Closed,
}

#[derive(Clone)]
pub enum HexDirection {
  NorthEast,
  East,
  SouthEast,
  SouthWest,
  West,
  NorthWest
}

impl HexDirection {
  pub fn opposite(&self) -> HexDirection {
    match self {
      HexDirection::NorthEast => {
        HexDirection::SouthWest
      },
      HexDirection::East => {
        HexDirection::West
      },
      HexDirection::SouthEast => {
        HexDirection::NorthWest
      },
      HexDirection::SouthWest => {
        HexDirection::NorthEast
      },
      HexDirection::West => {
        HexDirection::East
      },
      HexDirection::NorthWest => {
        HexDirection::SouthEast
      }
    }
  }
}

#[derive(Clone)]
pub struct Layout {
  origin: Vector2<f32>,
  size: Vector2<f32>,
  path: Vec<Vector2<f32>>,
}

impl Layout {
  pub fn new(origin: Vector2<f32>, size: Vector2<f32>) -> Layout {
    Layout {
      origin,
      size,
      path: Vec::new(),
    }
  }
  
  pub fn set_origin(&mut self, new_origin: Vector2<f32>) {
    self.origin = new_origin;
  }
  
  pub fn get_origin(&self) -> Vector2<f32> {
    self.origin
  }
  
  fn round_to_nearest_hex(q: f32, r: f32) -> Hexagon {
    let s = -q-r;
    
    let mut rnd_r = r.round();
    let mut rnd_q = q.round();
    let rnd_s = s.round();
    
    let r_diff = (rnd_r - r).abs();
    let q_diff = (rnd_q - q).abs();
    let s_diff = (rnd_s - s).abs();
    
    if r_diff > q_diff && r_diff > s_diff {
      rnd_r = -rnd_q-rnd_s;
    } else if q_diff > s_diff {
      rnd_q = -rnd_r-rnd_s;
    }
    
    Hexagon::new(rnd_q as i32, rnd_r as i32, "".to_string())
  }
  
  pub fn calculate_path(hexagons: &mut Vec<Hexagon>) -> Vec<u32> {
    let mut start_idx = 0;
    let mut end_idx = 0;
    
    for i in 0..hexagons.len() {
      if hexagons[i].is_start() {
        start_idx = i;
      }
      
      if hexagons[i].is_end() {
        end_idx = i;
      }
    }
    
    let mut came_from = Vec::with_capacity(hexagons.len());
    for _ in 0..hexagons.len() {
      came_from.push(None);
    }
    
    let mut frontier = Vec::new();
    frontier.push(start_idx);
    
    while frontier.len() != 0 {
      let current = frontier.remove(0);
      
      for next in &Hexagon::all_neigbors(hexagons[current].clone()) {
        let mut next_idx: i32 = -1;
        for i in 0..hexagons.len() {
          if Hexagon::hex_equals(next.clone(), hexagons[i].clone()) {
            if hexagons[i].is_path() {
              next_idx = i as i32;
            }
          }
        }
        
        if next_idx == -1 {
          continue;
        }
        
        if came_from[next_idx as usize].is_none() {
          frontier.push(next_idx as usize);
          came_from[next_idx as usize] = Some(current);
        }
      }
    }
    
    let mut current = end_idx;
    let mut path = Vec::new();
    while current != start_idx {
      path.push(current as u32);
      current = came_from[current].unwrap();
    }
    path.push(start_idx as u32);
    path.reverse();
    
    path
  }
  
  pub fn pixel_to_hex(&self, pixel: Vector2<f32>) -> Hexagon {
    let pt = Vector2::new((pixel.x - self.origin.x) / self.size.x,
                          (pixel.y - self.origin.y) / self.size.y);
    
    let q = B0 * pt.x + B1 * pt.y;
    let r = B2 * pt.x + B3 * pt.y;
    
    Layout::round_to_nearest_hex(q, r)
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
  hex_type: HexagonType,
}

impl Hexagon {
  pub fn new(q: i32, r: i32, model: String) -> Hexagon {
    Hexagon {
      position: Vector3::new(q, r, -q-r),
      model,
      hex_type: HexagonType::Open,
    }
  }
  
  pub fn set_type(&mut self, hex_type: HexagonType) {
    self.hex_type = hex_type;
  }
  
  pub fn is_path(&self) -> bool {
    self.hex_type == HexagonType::Path || self.is_start() || self.is_end()
  }
  
  pub fn is_start(&self) -> bool {
    self.hex_type == HexagonType::Start
  }
  
  pub fn is_end(&self) -> bool {
    self.hex_type == HexagonType::End
  }
  
  pub fn is_highlighted(&self) -> bool {
    self.model == "BlueHexagon".to_string()
  }
  
  pub fn plain(&mut self) {
    self.model = "Hexagon".to_string();
  }
  
  pub fn highlight(&mut self) {
    self.model = "BlueHexagon".to_string();
  }
  
  pub fn set_as_start(&mut self) {
    self.model = "RedHexagon".to_string();
    self.hex_type = HexagonType::Start;
  }
  
  pub fn set_as_end(&mut self) {
    self.model = "PurpleHexagon".to_string();
    self.hex_type = HexagonType::End;
  }
  
  pub fn set_as_path(&mut self) {
    self.model = "GreenHexagon".to_string();
    self.hex_type = HexagonType::Path;
  }
  
  pub fn is_open(&self) -> bool {
    self.hex_type == HexagonType::Open
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
  
  pub fn draw_hologram(&self, map: &Map, layout: &Layout, height: f32, draw_calls: &mut Vec<DrawCall>) {
    let position = layout.hex_to_pixel(self.clone());
    
    let radius = map.get_radius();
    
    draw_calls.push(DrawCall::draw_hologram_model(Vector3::new(position.x, height, position.y),
                                           Vector3::new(radius as f32/4.0, height, radius as f32/4.0),
                                           Vector3::new(0.0, 90.0, 0.0), 
                                           self.model.to_string()));
  }
  
  pub fn draw(&self, map: &Map, layout: &Layout, height: f32, draw_calls: &mut Vec<DrawCall>) {
    let position = layout.hex_to_pixel(self.clone());
    
    let radius = map.get_radius();
    
    draw_calls.push(DrawCall::draw_model(Vector3::new(position.x, height, position.y),
                                           Vector3::new(radius as f32/4.0, height, radius as f32/4.0),
                                           Vector3::new(0.0, 90.0, 0.0), 
                                           self.model.to_string()));
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
  
  pub fn all_neigbors(hexagon: Hexagon) -> Vec<Hexagon> {
    let mut neigbors = Vec::with_capacity(6);
    
    neigbors.push(Hexagon::hex_neigbor(hexagon.clone(), HexDirection::NorthEast));
    neigbors.push(Hexagon::hex_neigbor(hexagon.clone(), HexDirection::East));
    neigbors.push(Hexagon::hex_neigbor(hexagon.clone(), HexDirection::SouthEast));
    neigbors.push(Hexagon::hex_neigbor(hexagon.clone(), HexDirection::SouthWest));
    neigbors.push(Hexagon::hex_neigbor(hexagon.clone(), HexDirection::West));
    neigbors.push(Hexagon::hex_neigbor(hexagon, HexDirection::NorthWest));
    
    neigbors
  }
  
  pub fn generate_hexagon_range(radius: i32, texture: String) -> Vec<Hexagon> {
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
    for q in -radius..radius+1 {
      let r1 = (-radius).max(-q - radius);
      let r2 = radius.min(-q + radius);
      
      for r in r1..r2+1 {
        let dist = Hexagon::hex_distance(Hexagon::new(0, 0, "".to_string()), Hexagon::new(q, r, "".to_string()))%4;
        hexagons.push(Hexagon::new(q, r, texture.to_string()));
      }
    }
    
    hexagons
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

