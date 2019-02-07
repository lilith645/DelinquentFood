use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector3};

pub struct Map {
  size: Vector2<u32>,
  path: Vec<Vector2<u32>>,
}

impl Map {
  pub fn new() -> Map {
  // let map_1 = include_bytes!("../../resources/Maps/testmap.ini");
    
    let mut path = Vec::new();
    
    let mut width = 0;
    let mut height = 0;
    
    let mut x = 0;
    let mut y = 0;
    
    if let Ok(f) = File::open("./resources/Maps/testmap.ini") {
      println!("Settings file exists");
      let f = BufReader::new(f);
      
      for line in f.lines() {
        let line = line.expect("Unable to read line");
        let v: Vec<&str> = line.split(" ").collect();
        if width == 0 {
          width = v[0].parse::<u32>().unwrap();
        }
        
        if height == 0 {
          height = v[1].parse::<u32>().unwrap();
          continue;
        }
        
        for i in 0..width as usize {
          if v[i] ==  "0" {
            path.push(Vector2::new(x,y));
          }
          
          x += 1;
        }
        
        y += 1;
        x %= width;
        y %= height;
      }
    } else {
      panic!("Cant find map file");
    }
    
    Map {
      size: Vector2::new(width, height),
      path,
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(Vector3::new(0.0, 0.0, 0.0), Vector3::new(self.size.x as f32, 1.0, self.size.y as f32), Vector3::new(0.0, 0.0, 0.0), "Floor".to_string()));
    
    for pos in self.path.iter() {
      draw_calls.push(DrawCall::draw_model(Vector3::new(pos.x as f32*7.5-(self.size.x as f32*0.5*7.5).floor(), 
                                                        0.1, 
                                                        pos.y as f32*7.5-(self.size.y as f32*0.5*7.5).floor()), 
                                           Vector3::new(1.0, 1.0, 1.0), 
                                           Vector3::new(0.0, 0.0, 0.0), 
                                           "FloorPath".to_string()));
    }
  }
}
