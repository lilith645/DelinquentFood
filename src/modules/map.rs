use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::modules::hexagon::Hexagon;
use crate::modules::hexagon::Layout;
use crate::modules::hexagon::HexagonType;

use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct Map {
  radius: i32,
  layout: Layout,
  path: Vec<u32>,
  map: Vec<Hexagon>,
}

impl Map {
  pub fn new() -> Map {
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
    let mut radius: i32 = 0;
    let mut offset = 1;
    let mut y = 0;
    
    if let Ok(f) = File::open("./resources/Maps/MediumMap.ini") {
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
              if Hexagon::hex_equals(hexagons[i].clone(), hex.clone()) {
                hex_idx = i;
                break;
              }
            }
            
            if v[i] == "s" {
              hexagons[hex_idx].set_as_start();
            }
            
            if v[i] == "0" {
              hexagons[hex_idx].set_as_path();
            }
            
            if v[i] == "e" {
              hexagons[hex_idx].set_as_end();
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
    
    let layout = Layout::new(Vector2::new(0.0, 0.0), Vector2::new(radius as f32, radius as f32));
    
    let path = Layout::calculate_path(&mut hexagons);
    
    Map {
      radius: radius,
      layout,
      path,
      map: hexagons,
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for hexagon in &self.map {
      let height = {
        if hexagon.is_path() {
          0.2
        } else {
          1.0
        }
      };
      hexagon.draw(&self, &self.layout, height, draw_calls);
    }
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
  
  pub fn pixel_to_hex(&self, pix_x: f32, pix_y: f32) -> Hexagon {
    self.layout.pixel_to_hex(Vector2::new(pix_x, pix_y))
  }
  
  pub fn tile_position_from_index(&self, idx: usize) -> Vector2<f32> {
    self.layout.hex_to_pixel(self.map[idx].clone())
  }
  
  pub fn get_tile_position(&self, q: i32, r: i32) -> Vector2<f32> {
    self.layout.hex_to_pixel(Hexagon::new(q, r, "".to_string()))
  }
  
  pub fn get_qr_from_index(&self, idx: usize) -> Vector2<i32> {
    Vector2::new(self.map[idx].q(), self.map[idx].r())
  }
  
  pub fn get_index_from_qr(&self, q: i32, r: i32) -> Option<usize> {
    let test_hex = Hexagon::new(q,r,"".to_string());
    
    let mut idx = None;
    
    for i in 0..self.map.len() {
      if Hexagon::hex_equals(test_hex.clone(), self.map[i].clone()) {
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
      if !hex.is_path() {
        is_valid = true;
      }
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
      if Hexagon::hex_equals(light_hex.clone(), hexagon.clone()) {
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
