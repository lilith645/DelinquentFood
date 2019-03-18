use crate::modules::food::{Food, Strawberry, Banana, Pineapple, Mushroom, Jelly, Cake};
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
    wave5.push((Box::new(Pineapple::new(offset+1, food_pos, path.clone(), tile_loc)) as Box<Food>, 0 as f32));
    //
    let mut wave6 = Vec::new();
    wave6.push((Box::new(Cake::new(0, food_pos, path.clone(), tile_loc)) as Box<Food>, 0 as f32));
    
    let mut wave7 = Vec::new();
    for i in 0..20 {
      wave7.push((Box::new(Strawberry::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, (i as f32*0.5)));
    }
    for i in 20..80 {
      wave7.push((Box::new(Pineapple::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.5));
    }
    
    let mut wave8 = Vec::new();
    wave8.push((Box::new(Cake::new(0, food_pos, path.clone(), tile_loc)) as Box<Food>, 0 as f32));
    for i in 0..25 {
      wave8.push((Box::new(Pineapple::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.25));
      wave8.push((Box::new(Banana::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*0.25));
    }
    
    let mut wave9 = Vec::new();
    wave9.push((Box::new(Cake::new(0, food_pos, path.clone(), tile_loc)) as Box<Food>, 0 as f32));
    for i in 1..31 {
      wave9.push((Box::new(Mushroom::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32*1.5));
    }
    
    let mut wave10 = Vec::new();
    for i in 0..60 {
      wave10.push((Box::new(Jelly::new(i, food_pos, path.clone(), tile_loc)) as Box<Food>, i as f32));
    }
    
    FoodStore {
      waves: vec!(wave1, wave2, wave3, wave4, wave5, wave6,wave7, wave8, wave9, wave10),
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
