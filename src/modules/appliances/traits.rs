use maat_graphics::DrawCall;
use maat_graphics::camera;

use crate::modules::food::Food;
use crate::modules::weapons::Weapon;
use crate::modules::map::Map;
use crate::modules::hexagon::{Layout, Hexagon};

use cgmath::{InnerSpace, Angle, Deg, Vector2, Vector3, Vector4};

#[derive(Clone)]
pub enum TargetPriority {
  First,
  Last,
  Close,
  Far,
  Strong,
  Weak,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Buff {
  Range,
  AttackSpeed,
  AttackDamage,
  SellPrice, // sells for more than standard 60%
  Pierce,
  LifeExpectancy,
}

#[derive(Clone)]
pub struct ApplianceData {
  pub position: Vector3<f32>,
  pub offset: Vector3<f32>,
  pub size: Vector3<f32>,
  pub rotation: Vector3<f32>,
  pub model: String,
  pub tile_location: Vector2<i32>,
  pub range: u32,
  pub charge: f32,
  pub fire_rate: f32,
  pub target: TargetPriority,
  pub buffs: Vec<Buff>,
  pub life_expectancy: i32,
  pub max_life_expectancy: i32,
  pub draw_range: bool,
  pub buy_cost: i32,
  pub directional_range: bool,
}

impl ApplianceData {
  pub fn new(tile: Vector2<i32>, size: Vector3<f32>, rotation: Vector3<f32>, model: String, life: i32, rng: u32, fire_rate: f32, cost: i32, directional_range: bool, map: &Map) -> ApplianceData {
    let position = map.get_tile_position(tile.x as i32, tile.y as i32);
    
    ApplianceData {
      position: Vector3::new(position.x, 0.0, position.y),
      offset: Vector3::new(0.0, 0.0, 0.0),
      size,
      rotation,
      model,
      tile_location: tile,
      range: rng,
      charge: 0.0,
      fire_rate,
      target: TargetPriority::First,
      buffs: Vec::new(),
      life_expectancy: life,
      max_life_expectancy: life,
      draw_range: false,
      buy_cost: cost,
      directional_range,
    }
  }
}

pub trait ApplianceClone {
  fn clone_appliance(&self) -> Box<Appliance>;
}

impl<T: 'static + Appliance + Clone> ApplianceClone for T {
  fn clone_appliance(&self) -> Box<Appliance> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Appliance> {
  fn clone(&self) -> Box<Appliance> {
    self.clone_appliance()
  }
}

pub trait Appliance: ApplianceClone {
  fn data(&self) -> &ApplianceData;
  fn mut_data(&mut self) -> &mut ApplianceData;
  
  fn update(&mut self, foods: &mut Vec<Box<Food>>, weapons: &mut Vec<Box<Weapon>>, model_sizes: &mut Vec<(String, Vector3<f32>)>, map: &Map, delta_time: f32) -> Vec<(Buff, Vector2<i32>, u32)>;
  
  fn fire(&mut self);
  
  fn apply_effect(&self);
  fn remove_effects(&self);
  
  fn upgrade(&mut self);
  
  fn buy_cost(&self) -> i32 {
    self.data().buy_cost
  }
  
  fn sell_price(&self) -> i32 {
    let modifier = {
      if self.data().buffs.contains(&Buff::SellPrice) {
        0.8
      } else {
        0.6
      }
    };
    ((self.data().buy_cost as f32*modifier)*(self.current_life_expectancy() as f32/self.max_life_expectancy() as f32)).ceil() as i32
  }
  
  fn clean_cost(&self) -> i32 {
    ((self.max_life_expectancy() - self.current_life_expectancy()) as f32 * (self.buy_cost() as f32*0.6)*(1.0/self.max_life_expectancy() as f32)).ceil() as i32
  }
  
  fn upgrade_cost(&self) -> i32;
  fn sell(&self) -> i32;
  
  fn get_position(&self) -> Vector3<f32> {
    self.data().position
  }
  
  fn get_range(&self) -> u32 {
    self.data().range + if self.data().buffs.contains(&Buff::Range) { 1 } else { 0 }
  }
  
  fn get_qr_location(&self) -> Vector2<i32> {
    self.data().tile_location
  }
  
  fn decrease_life_expectancy(&mut self) {
    self.mut_data().life_expectancy -= 1;
  }
  
  fn current_life_expectancy(&self) -> i32 {
    self.data().life_expectancy
  }
  
  fn max_life_expectancy(&self) -> i32 {
    self.data().max_life_expectancy + if self.data().buffs.contains(&Buff::LifeExpectancy) { 1 } else { 0 }
  }
  
  fn get_fire_rate(&self) -> f32 {
    self.data().fire_rate * if self.data().buffs.contains(&Buff::AttackSpeed) { 0.8 } else { 1.0 }
  }
  
  fn clean(&mut self) {
    self.mut_data().life_expectancy = self.max_life_expectancy();
  }
  
  fn moved_tiles(&mut self, distance: i32) {
    self.mut_data().life_expectancy -= distance;
  }
  
  fn get_targeting(&self) -> TargetPriority {
    self.data().target.clone()
  }
  
  fn set_targeting(&mut self, new_target: TargetPriority) {
    self.mut_data().target = new_target;
  }
  
  fn apply_buff(&mut self, buff: Buff) {
    if !self.data().buffs.contains(&buff) {
      self.mut_data().buffs.push(buff);
    }
  }
  
  fn rotate_towards(&self, position: Vector3<f32>, food: &Box<Food>, angle_offset: f32) -> f32 {
    let loc = food.get_location();
    let direction = Vector2::new(loc.x-position.x, loc.y-position.z).normalize();
    let mut angle = Deg::atan2(direction.x, direction.y);
    
    angle.0 as f32+angle_offset
  }
  
  fn set_qr_location(&mut self, q: i32, r: i32, map: &Map) {
    let pos = map.get_tile_position(q, r);
    self.mut_data().tile_location = Vector2::new(q,r);
    self.mut_data().position.x = pos.x;
    self.mut_data().position.z = pos.y;
  }
  
  fn add_weapon_modifiers(&self, weapon: &mut Box<Weapon>) {
    if self.data().buffs.contains(&Buff::Pierce) { 
      weapon.add_pierce(2);
    }
    if self.data().buffs.contains(&Buff::AttackDamage) { 
      weapon.damage_multiplier(0.5);
    }
  }
  
  fn get_prioritised_food(&self, foods: &mut Vec<Box<Food>>, map: &Map) -> Option<Box<Food>> {
    let mut food = None;
    
    if foods.len() > 0 {
      let mut food_in_range = Vec::new();
      let mut food_distances = Vec::new();
      
      for food in foods {
        let food_pos = food.get_location();
        let other_hex = map.pixel_to_hex(food_pos);
        let hex = Hexagon::new(self.data().tile_location.x, self.data().tile_location.y, "".to_string());
        
        let dist = Hexagon::hex_distance(hex.clone(), other_hex.clone());
      
        if dist <= self.get_range() as i32 {
          if self.data().directional_range && self.get_range() > 1 {
            if !Hexagon::is_on_same_axis(hex, other_hex) {
              continue;
            }
          }
          food_in_range.push(food.clone());
          food_distances.push(dist);
        }
      }
      
      if food_in_range.len() == 0 {
        return food;
      }
      
      match &self.data().target {
        TargetPriority::First => {
          food = Some(food_in_range[0].clone());
        },
        TargetPriority::Last => {
          food = Some(food_in_range[food_in_range.len()-1].clone());
        },
        TargetPriority::Close => {
          let mut idx = 0;
          let mut closest_distance = food_distances[idx];
          for i in 1..food_distances.len() {
            let other_dist = food_distances[i];
            
            if other_dist < closest_distance {
              idx = i;
              closest_distance = other_dist;
            }
          }
          
          food = Some(food_in_range[idx].clone());
        },
        TargetPriority::Far => {
          let mut idx = 0;
          let mut furthest_distance = food_distances[idx];
          for i in 1..food_distances.len() {
            let other_dist = food_distances[i];
            
            if other_dist > furthest_distance {
              idx = i;
              furthest_distance = other_dist;
            }
          }
          
          food = Some(food_in_range[idx].clone());
        },
        TargetPriority::Strong => {
          let mut idx = 0;
          let mut most_health = food_in_range[idx].get_health();
          for i in (1..food_in_range.len()).rev() {
            let other_food_health = food_in_range[i].get_health();
            if other_food_health > most_health {
              most_health = other_food_health;
              idx = i;
            }
          }
          food = Some(food_in_range[idx].clone());
        },
        TargetPriority::Weak => {
          let mut idx = 0;
          let mut least_health = food_in_range[idx].get_health();
          for i in (1..food_in_range.len()).rev() {
            let other_food_health = food_in_range[i].get_health();
            if other_food_health < least_health {
              least_health = other_food_health;
              idx = i;
            }
          }
          food = Some(food_in_range[idx].clone());
        },
      }
    }
    
    food
  }
  
  fn should_draw_range(&mut self, should_draw: bool) {
    self.mut_data().draw_range = should_draw;
  }
  
  fn draw_range_coloured(&self, map: &Map, colour: Vector3<f32>, draw_calls: &mut Vec<DrawCall>) {
    let mut layout = map.get_layout();
    let new_origin = Vector2::new(self.data().position.x, self.data().position.z);
    layout.set_origin(new_origin);
    
    // draw hexagons
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
   
    let radius = self.get_range() as i32;
    let hexagons = {
      if self.data().directional_range {
        Hexagon::generate_directional_hexagon_range(radius, "PurpleHexagon".to_string())
      } else {
        Hexagon::generate_hexagon_range(radius, "PurpleHexagon".to_string())
      }
    };
    
    for hexagon in hexagons {
      let height = 1.2;
      let y_pos = 0.0;
      hexagon.draw_hologram_coloured(map, &layout, y_pos, height, colour, draw_calls);
    }
  }
  
  fn draw_range(&self, map: &Map, valid: bool, draw_calls: &mut Vec<DrawCall>) {
    let mut layout = map.get_layout();
    let new_origin = Vector2::new(self.data().position.x, self.data().position.z);
    layout.set_origin(new_origin);
    
    // draw hexagons
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
   
    let radius = self.get_range() as i32;
    let hexagons = {
      if self.data().directional_range {
        Hexagon::generate_directional_hexagon_range(radius, "PurpleHexagon".to_string())
      } else {
        Hexagon::generate_hexagon_range(radius, "PurpleHexagon".to_string())
      }
    };
    
    for hexagon in hexagons {
      let height = 1.2;
      let y_pos = 0.0;
       if valid {
         hexagon.draw_hologram_coloured(map, &layout, y_pos, height, Vector3::new(0.0, 1.0, 0.0), draw_calls);
       } else {
         hexagon.draw_hologram_coloured(map, &layout, y_pos, height, Vector3::new(1.0, 0.0, 0.0), draw_calls);
       }
    }
  }
  
  fn draw_hologram_invalid(&self, map: &Map, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::add_instanced_hologram_model_overwrite_colour(self.data().model.to_string(), self.data().position+self.data().offset, self.data().size, self.data().rotation, Vector3::new(1.0, 0.0, 0.0)));
    
    if self.data().draw_range {
      self.draw_range(map, false, draw_calls);
    }
  }
  
  fn draw_hologram(&self, map: &Map, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::add_instanced_hologram_model_overwrite_colour(self.data().model.to_string(), self.data().position+self.data().offset, self.data().size, self.data().rotation, Vector3::new(0.0, 1.0, 0.0)));
    
    if self.data().draw_range {
      self.draw_range(map, true, draw_calls);
    }
  }
  
  fn draw(&self, map: &Map, camera: &camera::Camera, window_dim: Vector2<f32>, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::add_instanced_model(self.data().model.to_string(), self.data().position+self.data().offset, self.data().size, self.data().rotation));
    
    if self.data().draw_range {
      self.draw_range(map, true, draw_calls);
    }
    
    let cam_pos = camera.get_position();
    let distance = (self.data().position-cam_pos).magnitude();
    let indicator_size = 114.0/distance*10.0;
    let offset = 114.0/distance*-8.0;
    let target_text_size = 114.0/distance*160.0;
    let target_offset = 114.0/distance*-38.0;
    
    let screen_coords = camera.world_to_screen_coords(self.data().position+self.data().offset, window_dim);
    
    let x_offset = -(self.max_life_expectancy() as f32 * 0.5 * indicator_size) + indicator_size*0.5;
    for i in 0..self.max_life_expectancy() {
      let texture = {
        if i+1 > self.current_life_expectancy() {
          "LifeIndicatorEmpty".to_string()
        } else {
          "LifeIndicatorFull".to_string()
        }
      };
      draw_calls.push(
        DrawCall::draw_textured(screen_coords+Vector2::new(x_offset+i as f32 * indicator_size, offset-self.data().offset.y), 
                                Vector2::new(indicator_size, indicator_size),
                                90.0,
                                texture)
      );
    }
    
    let target_name;
    match self.get_targeting() {
      TargetPriority::First => {
        target_name = "First".to_string();
      },
      TargetPriority::Last => {
        target_name = "Last".to_string();
      },
      TargetPriority::Close => {
        target_name = "Close".to_string();
      },
      TargetPriority::Far => {
        target_name = "Far".to_string();
      },
      TargetPriority::Strong => {
        target_name = "Strong".to_string();
      },
      TargetPriority::Weak => {
        target_name = "Weak".to_string();
      },
    }
    
    draw_calls.push(DrawCall::draw_text_basic_centered(screen_coords+Vector2::new(0.0, target_offset-self.data().offset.y), 
                                           Vector2::new(target_text_size, target_text_size), 
                                           Vector4::new(0.076078431, 1.0, 0.94745098, 1.0), 
                                           "(".to_owned() + &target_name + &")".to_string(), 
                                           "Arial".to_string()));
  }
}
