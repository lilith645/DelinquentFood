use crate::modules::food::Food;
use crate::modules::map::Map;


use cgmath::{Vector3};

type Wave = Vec<(Food, f32)>;

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
    let strawberry = Food::new(0, Vector3::new(food_pos.x, 0.0, food_pos.y), 20, "Strawberry".to_string(), path, tile_loc);
    
    let mut wave1 = Vec::new();
    for i in 0..40 {
      wave1.push((strawberry.clone(), i as f32*1.0));
    }
    let mut wave2 = Vec::new();
    for i in 0..120 {
      wave2.push((strawberry.clone(), i as f32*0.5));
    }
    
    FoodStore {
      waves: vec!(wave1, wave2),
      current_idx: 0,
      current_wave: 0,
      wave_delta: 0.0,
    }
  }
  
  pub fn wave_number(&self) -> usize {
    self.current_wave
  }
  
  pub fn next_wave(&mut self) {
    if self.current_wave < self.waves.len() {
      if self.waves[self.current_wave].len() == self.current_idx {
        self.current_wave += 1;
        self.current_idx = 0;
        self.wave_delta = 0.0;
      }
    }
  }
  
  pub fn update(&mut self, delta_time: f32) -> Option<Food> {
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
