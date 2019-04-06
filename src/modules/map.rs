use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::modules::hexagon::Hexagon;
use crate::modules::hexagon::Layout;
use crate::modules::hexagon::{HexDirection, HexagonType};

use maat_graphics::DrawCall;

use rand;
use rand::Rng;
use cgmath::{InnerSpace, Vector2, Vector3};

const TILE_DEFAULT_HEIGHT: f32 = 0.0;
const TILE_MAX_HEIGHT: f32 = 200.0;
const TILE_MIN_HEIGHT: f32 = -700.0;
const TILE_SPEED: f32 = 500.0;

#[derive(Clone)]
pub struct Map {
  radius: i32,
  layout: Layout,
  path: Vec<u32>,
  map: Vec<Hexagon>,
  is_ready: bool,
  resetting: bool,
  tile_delta: f32,
}

impl Map {
  pub fn new_random_map(radius: i32, rng: &mut rand::prelude::ThreadRng) -> Map {
    let mut radius = radius;
    if radius < 2 {
      radius = 2;
    }
    let mut start_hex: usize = 0;
    let mut current_hex: usize = 0;
    let mut end_hex: usize = 0;
    
    let mut hexagons = Hexagon::generate_hexagon_range(radius, "Hexagon".to_string());
    
    let start_q = radius-2;
    let start_r = -radius;
    let end_q = -(radius-2);
    let end_r = radius;
    
    for i in 0..hexagons.len() {
      if hexagons[i].q() == start_q && hexagons[i].r() == start_r {
        current_hex = i;
        continue;
      }
      if hexagons[i].q() == end_q && hexagons[i].r() == end_r {
        end_hex = i;
        continue;
      }
    }
    
    hexagons[current_hex].set_as_start();
    hexagons[end_hex].set_as_end();
    
    let mut found_end = false;
    
    while !found_end {
      let direction: u32 = (rng.gen::<f32>()*6.0) as u32;
      
      let mut neighbour: Hexagon = hexagons[current_hex].clone();
      match direction {
        0 => {
          neighbour = Hexagon::hex_neighbour(&hexagons[current_hex], &HexDirection::NorthEast);
        },
        1 => {
          neighbour = Hexagon::hex_neighbour(&hexagons[current_hex], &HexDirection::East);
        },
        2 => {
          neighbour = Hexagon::hex_neighbour(&hexagons[current_hex], &HexDirection::SouthEast);
        },
        3 => {
          neighbour = Hexagon::hex_neighbour(&hexagons[current_hex], &HexDirection::SouthWest);
        },
        4 => {
          neighbour = Hexagon::hex_neighbour(&hexagons[current_hex], &HexDirection::West);
        },
        5 => {
          neighbour = Hexagon::hex_neighbour(&hexagons[current_hex], &HexDirection::NorthWest);
        },
        _ => {},
      }
      
      for i in 0..hexagons.len() {
        if neighbour.q() == hexagons[i].q() && neighbour.r() == hexagons[i].r() {
          if hexagons[i].is_start() {
            break;
          }
          
          if hexagons[i].is_end() {
            found_end = true;
            break;
          }
          
          hexagons[i].set_as_path();
          current_hex = i;
          break;
        }
      }
    }
    
    let layout = Layout::new(Vector2::new(0.0, 0.0), Vector2::new(8.0, 8.0));
    
    let path = Layout::calculate_path(&mut hexagons);
    for i in 0..hexagons.len() {
      if !path.contains(&(i as u32)) {
        if !hexagons[i].is_start() && !hexagons[i].is_end() {
          hexagons[i].plain();
        }
      }
    }
    
    Map {
      radius,
      layout,
      path,
      map: hexagons,
      is_ready: false,
      resetting: false,
      tile_delta: TILE_MAX_HEIGHT,
    }
  }
  
  pub fn new(map_name: String) -> Map {
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
    let mut radius: i32 = 0;
    let mut offset = 1;
    let mut y = 0;
    
    if let Ok(f) = File::open("./resources/Maps/".to_owned() + &(map_name)) {
      println!("Settings file exists");
      let f = BufReader::new(f);
      
      for line in f.lines() {
        let line = line.expect("Unable to read line");
        let v: Vec<&str> = line.split(" ").collect();
        if radius == 0 {
          radius = v[0].parse::<i32>().unwrap();
          y = -radius;
          hexagons = Hexagon::generate_hexagon_range(radius, "Hexagon".to_string());
          continue;
        }
        
        let mut x = 0;
        for i in 0..v.len() {
          if v[i] != "" {
            x += 1;
            let q = x as i32- offset;
            let r = y as i32;
            
            let hex = Hexagon::new(q,r,"".to_string());
            
            let mut hex_idx = 0;
            
            for i in 0..hexagons.len() {
              if Hexagon::hex_equals(&hexagons[i], &hex) {
                hex_idx = i;
                break;
              }
            }
            
            if v[i] == "s" {
              hexagons[hex_idx].set_as_start();
              println!("start q {} r {}", hexagons[hex_idx].q(), hexagons[hex_idx].r());
            }
            
            if v[i] == "0" {
              hexagons[hex_idx].set_as_path();
            }
            
            if v[i] == "e" {
              hexagons[hex_idx].set_as_end();
              println!("end q {} r {}", hexagons[hex_idx].q(), hexagons[hex_idx].r());
            }
          }
        }
        y+=1;
        if y < 0 {
          offset += 1;
        } else if y == 0 {
          offset += 1;
        }
      }
    } else {
      panic!("Cant find map file");
    }
    
    let layout = Layout::new(Vector2::new(0.0, 0.0), Vector2::new(8.0, 8.0));
    
    let path = Layout::calculate_path(&mut hexagons);
    
    Map {
      radius: radius,
      layout,
      path,
      map: hexagons,
      is_ready: false,
      resetting: false,
      tile_delta: TILE_MAX_HEIGHT,
    }
  }
  
  pub fn update(&mut self, delta_time: f32) {
    if !self.is_ready && !self.resetting {
      self.tile_delta -= delta_time*TILE_SPEED;
      if self.tile_delta <= TILE_MIN_HEIGHT {
        self.tile_delta = TILE_DEFAULT_HEIGHT;
        self.is_ready = true;
      }
    }
    
    if self.resetting {
      self.tile_delta += delta_time*TILE_SPEED;
      if self.tile_delta >= TILE_MAX_HEIGHT {
        self.resetting = false;
      }
    }
  }
  
  pub fn draw(&self, hexagon_size: Vector3<f32>, cam_pos: Vector2<f32>, draw_calls: &mut Vec<DrawCall>) {
    let cam_hex = self.pixel_to_hex(cam_pos);
    
    for hexagon in &self.map {
      let mut y_pos = 0.0;
      
      let mut scale = 1.0;
      
      if !self.is_ready || self.resetting {
        let dist = cam_pos - self.layout.hex_to_pixel(&hexagon);
        let mag = dist.magnitude();
        y_pos = mag*3.0 + self.tile_delta;
        
        if y_pos < 0.0 {
          y_pos = 0.0;
        }
        
        scale = self.tile_delta/TILE_MIN_HEIGHT;
        if scale > 1.0 {
          scale = 1.0;
        }
      }
      
      let height = {
        if hexagon.is_path() {
          0.2
        } else {
          1.0
        }
      };
      hexagon.draw_scaled(&self, &self.layout, hexagon_size, y_pos, scale, height, draw_calls);
    }
  }
  
  pub fn reset(&mut self) {
    if !self.resetting {
      self.is_ready = false;
      self.resetting = true;
      self.tile_delta = TILE_MIN_HEIGHT;
      for hexagon in &mut self.map {
        if !hexagon.is_path() {
          hexagon.set_type(HexagonType::Open);
        }
      }
    }
  }
  
  pub fn is_ready(&self) -> bool {
    self.is_ready
  }
  
  pub fn get_radius(&self) -> i32 {
    self.radius
  }
  
  pub fn get_path(&self) -> Vec<u32> {
    self.path.clone()
  }
  
  pub fn get_layout(&self) -> Layout {
    self.layout.clone()
  }
  
  pub fn pixel_to_hex(&self, pixel: Vector2<f32>) -> Hexagon {
    self.layout.pixel_to_hex(pixel)
  }
  
  pub fn tile_position_from_index(&self, idx: usize) -> Vector2<f32> {
    self.layout.hex_to_pixel(&self.map[idx])
  }
  
  pub fn get_tile_position(&self, q: i32, r: i32) -> Vector2<f32> {
    self.layout.hex_to_pixel(&Hexagon::new(q, r, "".to_string()))
  }
  
  pub fn get_qr_from_index(&self, idx: usize) -> Vector2<i32> {
    Vector2::new(self.map[idx].q(), self.map[idx].r())
  }
  
  pub fn get_index_from_qr(&self, q: i32, r: i32) -> Option<usize> {
    let test_hex = Hexagon::new(q,r,"".to_string());
    
    let mut idx = None;
    
    for i in 0..self.map.len() {
      if Hexagon::hex_equals(&test_hex, &self.map[i]) {
        idx = Some(i as usize);
        break;
      }
    }
    
    idx
  }
  
  pub fn get_hex_from_qr(&self, q: i32, r: i32) -> Option<Hexagon> {
    let some_idx = self.get_index_from_qr(q, r);
    
    let mut hex = None;
    if let Some(idx) = some_idx {
      hex = Some(self.map[idx].clone());
    }
    
    hex
  }
  
  pub fn is_valid_qr(&self, q: i32, r: i32) -> bool {
    let mut is_valid = false;
    if let Some(hex) = self.get_hex_from_qr(q,r) {
      //if !hex.is_path() {
        is_valid = true;
     // }
    }
    
    is_valid
  }
  
  pub fn set_hexagon_type(&mut self, q: i32, r: i32, hex_type: HexagonType) {
    let some_idx = self.get_index_from_qr(q, r);
    
    if let Some(idx) = some_idx {
      self.map[idx].set_type(hex_type);
    }
  }
  
  pub fn highlight_hex(&mut self, light_hex: Hexagon) {
    for hexagon in &mut self.map {
      if Hexagon::hex_equals(&light_hex, &hexagon) {
        hexagon.highlight();
      }
    }
  }
  
  pub fn unhighlight_all_hexs(&mut self) {
    for hexagon in &mut self.map {
      if hexagon.is_highlighted() {
        hexagon.plain();
      }
    }
  }
}
