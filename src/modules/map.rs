use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::modules::hexagon::Hexagon;
use crate::modules::hexagon::Layout;

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
    
    if let Ok(f) = File::open("./resources/Maps/testmap.ini") {
      println!("Settings file exists");
      let f = BufReader::new(f);
      
      for line in f.lines() {
        let line = line.expect("Unable to read line");
        let v: Vec<&str> = line.split(" ").collect();
        if radius == 0 {
          radius = v[0].parse::<i32>().unwrap();
          y = -radius;
          for q in -radius..radius+1 {
            let r1 = (-radius).max(-q - radius);
            let r2 = radius.min(-q + radius);
            
            for r in r1..r2+1 {
             // let dist = Hexagon::hex_distance(Hexagon::new(0, 0, "".to_string()), Hexagon::new(q, r, "".to_string()))%4;
              let texture = "Hexagon".to_string();
              
              hexagons.push(Hexagon::new(q, r, texture.to_string()));
            }
          }
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
      let location = self.layout.hex_to_pixel(hexagon.clone());
      
      let position = Vector3::new(location.x, 0.1, location.y);
      let height = {
        if hexagon.is_path() {
          0.1
        } else {
          1.0
        }
      };
      draw_calls.push(DrawCall::draw_model(position,
                                           Vector3::new(self.radius as f32/4.0, height, self.radius as f32/4.0),
                                           Vector3::new(0.0, 90.0, 0.0), 
                                           hexagon.get_model()));
    }
  }
  
  pub fn get_radius(&self) -> i32 {
    self.radius
  }
  
  pub fn get_path(&self) -> Vec<u32> {
    self.path.clone()
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
  
  pub fn highlight_hex(&mut self, light_hex: Hexagon) {
    for hexagon in &mut self.map {
      if Hexagon::hex_equals(light_hex.clone(), hexagon.clone()) {
        hexagon.highlight();
      }
    }
  }
}
