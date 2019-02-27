use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::modules::hexagon::Hexagon;
use crate::modules::hexagon::HexDirection;
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
    let mut x = 0;
    let mut y = 0;
    
    let mut q = 0;
    let mut r = 0;
    
    if let Ok(f) = File::open("./resources/Maps/testmap.ini") {
      println!("Settings file exists");
      let f = BufReader::new(f);
      
      for line in f.lines() {
        let line = line.expect("Unable to read line");
        let v: Vec<&str> = line.split(" ").collect();
        if radius == 0 {
          radius = v[0].parse::<i32>().unwrap();
          q = 0;
          r = -1*radius;
          for q in -radius..radius+1 {
            let r1 = (-radius).max(-q - radius);
            let r2 = radius.min(-q + radius);
            
            for r in r1..r2+1 {
              let dist = Hexagon::hex_distance(Hexagon::new(0, 0, "".to_string()), Hexagon::new(q, r, "".to_string()))%4;
              let mut texture = "Hexagon".to_string();
              /*match dist {
                0 => { texture = "BlueHexagon".to_string(); },
                1 => { texture = "GreenHexagon".to_string(); },
                2 => { texture = "PurpleHexagon".to_string(); },
                3 => { texture = "RedHexagon".to_string(); },
                _ => {}
              }*/
              
              hexagons.push(Hexagon::new(q, r, texture.to_string()));
            }
          }
          continue;
        }
        /*
        let mut hex = Hexagon::new(q, r, "Hexagon".to_string());
        let s_q = q;
        
        for i in 0..v.len() {
          if v[i] == "0" {
            for j in 0..hexagons.len() {
              if Hexagon::hex_equals(hexagons[j].clone(), hex.clone()) {
                hexagons[j].set_as_path();
                break;
              }
            }
            
            q += 1;
            hex = Hexagon::new(q, r, "Hexagon".to_string());
          }
        }
        
        if r >= 0 {
          q = s_q;
        } else {
          q = s_q - 1;
        }
        
        r += 1;*/
        
       // println!("");
        /*
        if tiles_info_next {
          num_tiles = v[2].parse::<u32>().unwrap();
          
          for i in 3..num_tiles+3 {
            order.push(v[i as usize].parse::<u32>().unwrap());
          }
          
          tiles_info_next = false;
          continue;
        }*/
        
        /*for i in 0..width as usize {
          if v[i] ==  "0" {
            path.push(Vector2::new(x,y));
          }
          
          x += 1;
        }*/
        
        y += 1;
      }
    } else {
      panic!("Cant find map file");
    }
    
    for j in 0..hexagons.len() {
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(6, -8, "".to_string())) {
        hexagons[j].set_as_start();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(6, -7, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(6, -6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(5, -5, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(4, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(3, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(2, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, -5, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, -6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(0, -6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-1, -5, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-2, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-3, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-4, -3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-4, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-3, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-2, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-1, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(0, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(2, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(3, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(4, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(5, -3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(6, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(7, -4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(7, -3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(7, -2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(7, -1, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(6, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(5, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(4, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(3, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(2, 1, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, 1, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(0, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-1, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-2, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-3, 0, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-4, 1, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-5, 2, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-6, 3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-6, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-5, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-4, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-3, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-2, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-1, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(0, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, 3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(2, 3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(3, 3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(4, 3, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(3, 4, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(2, 5, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(1, 6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(0, 6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-1, 6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-2, 6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-3, 6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-4, 6, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-5, 7, "".to_string())) {
        hexagons[j].set_as_path();
      }
      if Hexagon::hex_equals(hexagons[j].clone(), Hexagon::new(-6, 8, "".to_string())) {
        hexagons[j].set_as_end();
      }
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
      draw_calls.push(DrawCall::draw_model(position,
                                           Vector3::new(self.radius as f32/4.0, 0.1, self.radius as f32/4.0),
                                           Vector3::new(0.0, 90.0, 0.0), 
                                           hexagon.get_model()));
    }
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
