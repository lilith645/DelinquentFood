use crate::modules::food::{Food, Strawberry, Banana, Pineapple, Cake};
use crate::modules::map::Map;

use cgmath::{Vector3};

type Wave = Vec<(Box<Food>, f32)>;

#[derive(Clone)]
pub struct FoodStore {
  waves: Vec<Wave>,
  current_idx: usize,
  current_wave: usize,
  wave_delta: f32,
}

impl FoodStore {
  pub fn new(map: &Map) -> FoodStore {
    let path = map.get_path();
    let food_pos = map.tile_position_from_index(path[0] as usize);
    let tile_loc = map.get_qr_from_index(path[0] as usize);
    
    // 83 dollars
    let mut wave1 = Vec::new();
    for i in 0..39 {
      wave1.push((Box::new(Banana::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32));
    }
    wave1.push((Box::new(Strawberry::new(40, food_pos, path.clone(), tile_loc)) as Box<Food>, 40.0));
    wave1.push((Box::new(Pineapple::new(41, food_pos, path.clone(), tile_loc)) as Box<Food>, 45.0));
    
    // 145 dollars
    let mut wave2 = Vec::new();
    for i in 0..5 {
      wave2.push((Box::new(Strawberry::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, ((i as f32*0.25))));
    }
    for i in 0..60 {
      wave2.push((Box::new(Banana::new(6+i, food_pos, path.clone(), tile_loc)) as Box<Food>, 1.25+i as f32*0.75));
    }
    
    // 280 dollars
    let mut wave3 = Vec::new();
    for i in 0..80 {
      if i%2 == 0 {
        wave3.push((Box::new(Banana::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
      } else {
        wave3.push((Box::new(Strawberry::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
      }
    }
    
    // 400 dollars
    let mut wave4 = Vec::new();
    for i in 0..80 {
      wave4.push((Box::new(Strawberry::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
    }
    
    // 800 dollars
    let mut wave5 = Vec::new();
    let mut offset = 0;
    
    let spacing = 0.3;
    
    for j in 0..3 {
      for i in 0..20 {
        wave5.push((Box::new(Strawberry::new(offset+i, food_pos, path.clone(), tile_loc)) as Box<Food>, offset as f32*spacing+i as f32*spacing));
      }
      offset += 20;
      for i in 0..50 {
        wave5.push((Box::new(Banana::new(offset+i, food_pos, path.clone(), tile_loc)) as Box<Food>, offset as f32*spacing+i as f32*spacing));
      }
      offset += 50;
    }
    
    let mut wave6 = Vec::new();
    wave6.push((Box::new(Cake::new(0, food_pos, path.clone(), tile_loc)) as Box<Food>, 0 as f32));
    
    FoodStore {
      waves: vec!(wave1, wave2, wave3, wave4, wave5, wave6),
      current_idx: 0,
      current_wave: 0,
      wave_delta: 0.0,
    }
  }
  
  pub fn wave_number(&self) -> usize {
    self.current_wave
  }
  
  pub fn next_wave(&mut self) -> bool {
    let mut next_wave_started = false;
    if self.current_wave < self.waves.len() {
      if self.waves[self.current_wave].len() == self.current_idx {
        next_wave_started = true;
        self.current_wave += 1;
        self.current_idx = 0;
        self.wave_delta = 0.0;
      }
    }
    
    next_wave_started
  }
  
  pub fn update(&mut self, delta_time: f32) -> Option<Box<Food>> {
    if self.current_wave < self.waves.len() {
      let wave = &mut self.waves[self.current_wave];
      
      if self.current_idx < wave.len() {
        if self.wave_delta >= wave[self.current_idx].1 {
          let food = wave[self.current_idx].0.clone();
          self.current_idx += 1;
          return Some(food);
        }
      }
      
      self.wave_delta += delta_time;
    }
    
    None
  }
}
