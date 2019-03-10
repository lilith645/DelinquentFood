use crate::modules::food::{Food, Strawberry, Banana};
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
    
    // 205 dollars
    let mut wave1 = Vec::new();
    for i in 0..39 {
      wave1.push((Box::new(Banana::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32));
    }
    wave1.push((Box::new(Strawberry::new(40, food_pos, path.clone(), tile_loc)) as Box<Food>, 40.0));
    
    // 300 dollars
    let mut wave2 = Vec::new();
    for i in 0..5 {
      wave2.push((Box::new(Strawberry::new(121+i, food_pos, path.clone(), tile_loc)) as Box<Food>, ((i as f32*0.25))));
    }
    for i in 0..60 {
      wave2.push((Box::new(Banana::new(6+i, food_pos, path.clone(), tile_loc)) as Box<Food>, 1.25+i as f32*0.75));
    }
    
    // 600 dollars
    let mut wave3 = Vec::new();
    for i in 0..80 {
      if i%2 == 0 {
        wave3.push((Box::new(Banana::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
      } else {
        wave3.push((Box::new(Strawberry::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
      }
    }
    
    // 800 dollars
    let mut wave4 = Vec::new();
    for i in 0..80 {
      wave4.push((Box::new(Strawberry::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
    }
    
    FoodStore {
      waves: vec!(wave1, wave2, wave3, wave4),
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
