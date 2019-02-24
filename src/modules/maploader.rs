use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::modules::hexagon::Hexagon;
use crate::modules::hexagon::HexDirection;
use crate::modules::hexagon::Layout;

use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector3};

pub struct Map {
  radius: i32,
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
        
        println!("");
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
        hexagons[j].set_as_path();
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
        hexagons[j].set_as_path();
      }
    }
    
    Map {
      radius: radius,
      map: hexagons,
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let size = Vector2::new(8.0, 8.0);
    
    let layout = Layout::new(Vector2::new(0.0, 0.0), size);
    
    for hexagon in &self.map {
      let location = layout.hex_to_pixel(hexagon.clone());
      
      let position = Vector3::new(location.x, 0.1, location.y);
      draw_calls.push(DrawCall::draw_model(position,
                                           Vector3::new(size.x/4.0, 0.1, size.y/4.0),
                                           Vector3::new(0.0, 90.0, 0.0), 
                                           hexagon.get_model()));
    }
  }
  
  /*
  pub fn get_tile_position(&self, x: u32, y: u32) -> Vector2<f32> {
    Vector2::new(-(self.size.x as f32*2.8) + (self.size.x as f32*0.8*((x) as f32-1.0)),
                 -(self.size.y as f32*2.8) + (self.size.y as f32*0.8*((y) as f32-1.0)))
  }
  
  pub fn get_path_position(&self, tile: usize) -> Vector2<f32> {
    Vector2::new(-(self.size.x as f32*2.8) + (self.size.x as f32*0.8*((self.path[self.order[tile] as usize-1].x) as f32-1.0)),
                 -(self.size.y as f32*2.8) + (self.size.y as f32*0.8*((self.path[self.order[tile] as usize-1].y) as f32-1.0)))
  }
  
  pub fn get_path_location(&self, tile: usize) -> Vector2<u32> {
    self.path[self.order[tile] as usize-1]
  }
  
  pub fn get_next_path(&self, tile: u32) -> u32 {
    if self.order.len()-2 < tile as usize {
      for i in 0..self.order.len() {
        if self.order[i] == 1 {
          return i as u32;
        }
      }
      
      return 0
    }
    
    tile + 1
  }*/
}
