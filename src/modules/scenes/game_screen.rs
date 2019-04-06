use maat_graphics::DrawCall;
use maat_graphics::camera;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::MenuScreen;

use crate::modules::food::Food;
use crate::modules::appliances::{Dishwasher, Fridge, MeatTenderizer, CoffeeMachine, SaltGrinder};
use crate::modules::appliances::traits::{Appliance, TargetPriority};
use crate::modules::weapons::{Weapon};
use crate::modules::hexagon::{Layout, Hexagon, HexagonType, HexDirection};
use crate::modules::thefoodstore::FoodStore;

use crate::modules::update::update_game;
use crate::modules::physics::collisions;
use crate::modules::map::Map;

use rand;
use rand::{thread_rng};

use cgmath::{InnerSpace, SquareMatrix, Matrix4, Point3, Deg, Vector2, Vector3, Vector4, PerspectiveFov};

const DEV: bool = false;

const DEFAULT_ZOOM: f32 = 1.0;
const DELTA_STEP: f32 = 0.01;

const START_MONEY: i32 = 300;
const BIN_CLEAN_COST: i32 = 700;

const CAMERA_DEFAULT_X: f32 = 83.93359;
const CAMERA_DEFAULT_Y: f32 = 128.62776;
const CAMERA_DEFAULT_Z: f32 = 55.85842;
const CAMERA_DEFAULT_PITCH: f32 = -62.27426;
const CAMERA_DEFAULT_YAW: f32 = 210.10083;
const CAMERA_DEFAULT_SPEED: f32 = 50.0;

enum MouseState {
  World,
  Ui,
  Placing,
}

pub struct GameScreen {
  data: SceneData,
  zoom: f32,
  escaped_pressed_last_frame: bool,
  space_pressed_last_frame: bool,
  t_pressed_last_frame: bool,
  f1_pressed_last_frame: bool,
  f2_pressed_last_frame: bool,
  f10_pressed_last_frame: bool,
  p_pressed_last_frame: bool,
  screen_offset: Vector2<f32>,
  camera: camera::Camera,
  rng: rand::prelude::ThreadRng,
  last_mouse_pos: Vector2<f32>,
  total_delta: f32,
  map: Map,
  appliances: Vec<Box<Appliance>>,
  foods: Vec<Box<Food>>,
  weapons: Vec<Box<Weapon>>,
  ray_position: Vector2<f32>,
  game_speed: i32,
  mouse_state: MouseState,
  bin: i32,
  placing_appliance: Option<Box<Appliance>>,
  selected_appliance: Option<usize>,
  valid_place: bool,
  the_food_store: FoodStore,
  money: i32,
  minimal_ui: bool,
}

impl GameScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>, map_name: String) -> GameScreen {
    println!("Game Screen");
    
    let mut camera = camera::Camera::default_vk();
    camera.set_position(Vector3::new(CAMERA_DEFAULT_X, CAMERA_DEFAULT_Y, CAMERA_DEFAULT_Z));
    camera.set_pitch(CAMERA_DEFAULT_PITCH);
    camera.set_yaw(CAMERA_DEFAULT_YAW);
    camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    
    let mut rng =  thread_rng();
    
    let map = Map::new_random_map(5, &mut rng);
    //let map = Map::new(map_name.to_string());
    
    let path = map.get_path();
    let food_pos = map.tile_position_from_index(path[0] as usize);
    let tile_loc = map.get_qr_from_index(path[0] as usize);
    let store = FoodStore::new(&map);
    
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      space_pressed_last_frame: false,
      t_pressed_last_frame: false,
      f1_pressed_last_frame: false,
      f2_pressed_last_frame: false,
      f10_pressed_last_frame: false,
      p_pressed_last_frame: false,
      screen_offset: Vector2::new(0.0, 0.0),
      camera,
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      map,
      appliances: Vec::new(),
      foods: vec!(),
      weapons: Vec::new(),
      ray_position: Vector2::new(0.0, 0.0),
      game_speed: 1,
      mouse_state: MouseState::World,
      bin: 0,
      placing_appliance: None,
      selected_appliance: None,
      valid_place: false,
      the_food_store: store,
      money: 300,
      minimal_ui: false,
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, screen_offset: Vector2<f32>, appliances: Vec<Box<Appliance>>, foods: Vec<Box<Food>>, map: Map, model_sizes: Vec<(String, Vector3<f32>)>, weapons: Vec<Box<Weapon>>, the_food_store: FoodStore, money: i32, game_speed: i32, bin: i32) -> GameScreen {
    
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      space_pressed_last_frame: false,
      t_pressed_last_frame: false,
      f1_pressed_last_frame: false,
      f2_pressed_last_frame: false,
      f10_pressed_last_frame: false,
      p_pressed_last_frame: false,
      screen_offset,
      camera,
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      map,
      appliances,
      foods,
      weapons,
      ray_position: Vector2::new(0.0, 0.0),
      game_speed,
      mouse_state: MouseState::World,
      bin,
      placing_appliance: None,
      selected_appliance: None,
      valid_place: false,
      the_food_store,
      money,
      minimal_ui: false,
    }
  }
  
  fn start_placing_tower(&mut self, mouse: Vector2<f32>, appliance: Box<Appliance>) {
    let mouse_ray = self.camera.mouse_to_world_ray(mouse, self.data.window_dim);
    
    let mut q = 0;
    let mut r = 0;
    if mouse_ray.y < 0.0 {
      let mut crnt_pos = self.camera.get_position();
      while crnt_pos.y > 0.0 {
        crnt_pos += mouse_ray;
      }
      crnt_pos -= mouse_ray;
      
      let pix_x = crnt_pos.x;
      let pix_y = crnt_pos.z;
      let clicked_hex = self.map.pixel_to_hex(Vector2::new(pix_x, pix_y));
      q = clicked_hex.q();
      r =  clicked_hex.r();
      self.valid_place = self.map.is_valid_qr(q,r);
    } else {
      self.valid_place = false;
    }
    
    let mut appliance = appliance;
    appliance.set_qr_location(q,r, &self.map);
    self.placing_appliance = Some(appliance);
    if let Some(appliance) = &mut self.placing_appliance {
      let foods = &mut self.foods;
      let weapons = &mut self.weapons;
      let m_sizes = &mut self.data.model_sizes;
      let map = &self.map;
      
      appliance.update(foods, weapons, m_sizes, map, 0.0);
      appliance.should_draw_range(true);
      if self.selected_appliance.is_some() {
        self.appliances[self.selected_appliance.unwrap()].should_draw_range(false);
      }
      self.selected_appliance = None;
    }
    self.mouse_state = MouseState::Placing;
  }
  
  pub fn update_keypresses(&mut self, delta_time: f32) {
    let mouse = self.data.mouse_pos;
    
    let escape_pressed = self.data().keys.escape_pressed();
    let one_pressed = self.data.keys.one_pressed();
    let two_pressed = self.data.keys.two_pressed();
    let three_pressed = self.data.keys.three_pressed();
    let four_pressed = self.data.keys.four_pressed();
    let five_pressed = self.data.keys.five_pressed();
    let mut w_pressed = self.data.keys.w_pressed();
    let mut a_pressed = self.data.keys.a_pressed();
    let mut s_pressed = self.data.keys.s_pressed();
    let mut d_pressed = self.data.keys.d_pressed();
    let c_pressed = self.data().keys.c_pressed();
    let r_pressed = self.data().keys.r_pressed();
    let f_pressed = self.data().keys.f_pressed();
    let m_pressed = self.data().keys.m_pressed();
    let p_pressed = self.data().keys.p_pressed();
    let x_pressed = self.data().keys.x_pressed();
    let k_pressed = self.data().keys.k_pressed();
    let b_pressed = self.data().keys.b_pressed();
    let v_pressed = self.data().keys.v_pressed();
    let f10_pressed = self.data().keys.f10_pressed();
    
    let t_pressed = self.data().keys.t_pressed();
    
    match self.update_controller_input() {
      (w, a, s, d, x_offset, y_offset) => {
        // First person setup
        if w { w_pressed = w; }
        if a { a_pressed = a; }
        if s { s_pressed = s; }
        if d { d_pressed = d; }
        self.camera.process_mouse_movement(x_offset, y_offset);
      }
    }
    
    self.last_mouse_pos = mouse;
    
    if p_pressed && !self.p_pressed_last_frame {
      if self.game_speed < 2 {
        self.game_speed = (self.game_speed+1)%2;
      } else {
        self.game_speed = 0;
      }
    }
    
    if w_pressed {
      self.camera.process_movement(camera::Direction::YAlignedForward, delta_time);
    }
    if a_pressed {
      self.camera.process_movement(camera::Direction::YAlignedLeft, delta_time);
    }
    if s_pressed {
      self.camera.process_movement(camera::Direction::YAlignedBackward, delta_time);
    }
    if d_pressed {
      self.camera.process_movement(camera::Direction::YAlignedRight, delta_time);
    }
    if r_pressed {
      self.camera.process_movement(camera::Direction::PositiveY, delta_time);
    }
    if f_pressed {
      self.camera.process_movement(camera::Direction::NegativeY, delta_time);
    }
    if one_pressed {
      self.start_placing_tower(mouse,
                               Box::new(Dishwasher::new(Vector2::new(0,0), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, 0.0), &self.map))
                              );
    }
    if two_pressed {
      self.start_placing_tower(mouse, 
                               Box::new(Fridge::new(Vector2::new(0,0), Vector3::new(3.0, 3.0, 3.0), Vector3::new(0.0, 0.0, 0.0), &self.map))
                              );
    }
    if three_pressed {
      self.start_placing_tower(mouse, 
                               Box::new(MeatTenderizer::new(Vector2::new(0,0), Vector3::new(3.0, 3.0, 3.0), Vector3::new(0.0, 0.0, 0.0), &self.map))
                              );
    }
    if four_pressed {
      self.start_placing_tower(mouse, 
                               Box::new(CoffeeMachine::new(Vector2::new(0,0), Vector3::new(0.3, 0.3, 0.3), Vector3::new(0.0, 0.0, 0.0), &self.map))
                              );
    }
    if five_pressed {
      self.start_placing_tower(mouse, 
                               Box::new(SaltGrinder::new(Vector2::new(0,0), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, -90.0), &self.map))
                              );
    }
    
    if b_pressed && self.bin > 0 {
      if self.money >= BIN_CLEAN_COST {
        self.money -= BIN_CLEAN_COST;
        self.bin = 0;
      }
    }
    
    if f10_pressed && !self.f10_pressed_last_frame {
      self.minimal_ui = !self.minimal_ui;
    }
    
    // reseting
    if k_pressed {
      self.map.reset();
      self.foods.clear();
      self.appliances.clear();
      self.weapons.clear();
      self.money = START_MONEY;
      self.placing_appliance = None;
      self.selected_appliance = None;
      self.bin = 0;
      self.game_speed = 1;
      self.total_delta = 0.0;
      self.the_food_store = FoodStore::new(&self.map);
    }
    
    if v_pressed || k_pressed {
      self.camera.set_position(Vector3::new(CAMERA_DEFAULT_X, CAMERA_DEFAULT_Y, CAMERA_DEFAULT_Z));
      self.camera.set_pitch(CAMERA_DEFAULT_PITCH);
      self.camera.set_yaw(CAMERA_DEFAULT_YAW);
      self.camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    }
    
    if let Some(idx) = self.selected_appliance {
      // Change target priority for selected appliance
      if t_pressed && !self.t_pressed_last_frame {
        match self.appliances[idx].get_targeting() {
          TargetPriority::First => {
            self.appliances[idx].set_targeting(TargetPriority::Last);
          },
          TargetPriority::Last => {
            self.appliances[idx].set_targeting(TargetPriority::Close);
          },
          TargetPriority::Close => {
            self.appliances[idx].set_targeting(TargetPriority::Far);
          },
          TargetPriority::Far => {
            self.appliances[idx].set_targeting(TargetPriority::Strong);
          },
          TargetPriority::Strong => {
            self.appliances[idx].set_targeting(TargetPriority::Weak);
          },
          TargetPriority::Weak => {
            self.appliances[idx].set_targeting(TargetPriority::First);
          },
        }
      }
      
      // Sell tower
      if x_pressed {
        self.money += self.appliances[idx].sell_price();
        let hex_location = self.appliances[idx].get_qr_location();
        let range = self.appliances[idx].get_range();
        self.map.set_hexagon_type(hex_location.x, hex_location.y, HexagonType::Open);
        
        let appliance_hex = Hexagon::new(hex_location.x, hex_location.y, "".to_string());
        let buffs = self.appliances[idx].update(&mut self.foods, &mut self.weapons, &mut self.data.model_sizes, &self.map, delta_time);
        
        let hexs = Hexagon::generate_hexagon_range(range as i32, "".to_string());
        for hex in &hexs {
          let t_hex = Hexagon::hex_add(&appliance_hex, &hex);
          for appliance in &mut self.appliances {
            let qr = appliance.get_qr_location();
            if qr.x == t_hex.q() && qr.y == t_hex.r() {
              for (buff, _, _) in &buffs {
                appliance.remove_buff(buff);
              }
            }
          }
        }
        
        self.appliances.remove(idx);
        self.selected_appliance = None;
      }
      
      // move tower
      if m_pressed {
        let mut appliance = self.appliances[idx].clone();
        appliance.should_draw_range(true);
        self.placing_appliance = Some(appliance);
        self.mouse_state = MouseState::Placing;
        
        let life = self.appliances[idx].current_life_expectancy();
        
        let mut hexagons: Vec<Hexagon> = Vec::new();
        let radius = life-1;
        let hexagons = Hexagon::generate_hexagon_range(radius, "PurpleHexagon".to_string());
        
        let qr = self.appliances[idx].get_qr_location();
        let appliance_hex = Hexagon::new(qr.x,qr.y, "".to_string());
        
        for hexagon in &hexagons {
          let hex = Hexagon::hex_add(&appliance_hex, hexagon);
          let q = hex.q();
          let r = hex.r();
          if self.map.is_valid_qr(q,r) {
            self.map.highlight_hex(hex);
          }
        }
      }
      // Clean tower
      if c_pressed {
        if self.money >= self.appliances[idx].clean_cost() {
          self.money -= self.appliances[idx].clean_cost();
          self.appliances[idx].clean();
        }
      }
    }
    
    self.escaped_pressed_last_frame = escape_pressed;
    self.t_pressed_last_frame = t_pressed;
    self.f10_pressed_last_frame = f10_pressed;
    self.p_pressed_last_frame = p_pressed;
  }
  
  pub fn update_controller_input(&mut self) -> (bool, bool, bool, bool, f32, f32) {
    let mut w_pressed = false;
    let mut a_pressed = false;
    let mut s_pressed = false;
    let mut d_pressed = false;
    
    if self.data.controller.dpad_up_pressed() {
      w_pressed = true;
    }
    if self.data().controller.dpad_down_pressed() {
      s_pressed = true;
    }
    if self.data().controller.dpad_left_pressed() {
      a_pressed = true;
    }
    if self.data().controller.dpad_right_pressed() {
      d_pressed = true;
    }
    
    let left_stick_pos = self.data.controller.left_stick_position();
    if left_stick_pos.y > 0.1 {
      w_pressed = true;
    }
    if left_stick_pos.y < -0.1 {
      s_pressed = true;
    }
    if left_stick_pos.x > 0.1 {
      d_pressed = true;
    }
    if left_stick_pos.x < -0.1 {
      a_pressed = true;
    }
    
    let right_stick_pos = self.data.controller.right_stick_position();
    
    (w_pressed, a_pressed, s_pressed, d_pressed, -right_stick_pos.x, right_stick_pos.y)
  }
  
  pub fn update_world(&mut self, delta_time: f32) {
    let left_clicked = self.data.left_mouse;
    let right_clicked = self.data.right_mouse;
    let mouse = self.data.mouse_pos;
    let escape_pressed = self.data().keys.escape_pressed();
    
    if self.escaped_pressed_last_frame && !escape_pressed {
      self.escaped_pressed_last_frame = false;
      self.mut_data().next_scene = true;
    }
    
    if right_clicked {
      if self.selected_appliance.is_some() {
        self.selected_appliance = None;
      }
    }
    
    if left_clicked {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        let x_offset = self.last_mouse_pos.x - mouse.x;
        let y_offset = mouse.y - self.last_mouse_pos.y;
        self.camera.process_mouse_movement(x_offset, y_offset);
      }
      
      let mouse_ray = self.camera.mouse_to_world_ray(mouse, self.data.window_dim);
      if mouse_ray.y < 0.0 {
        let mut crnt_pos = self.camera.get_position();
        while crnt_pos.y > 0.0 {
          crnt_pos += mouse_ray;
        }
        crnt_pos -= mouse_ray;
        
        let pix_x = crnt_pos.x;
        let pix_y = crnt_pos.z;
        let clicked_hex = self.map.pixel_to_hex(Vector2::new(pix_x, pix_y));
        let q = clicked_hex.q();
        let r =  clicked_hex.r();
        
        let mut found_appliance = false;
        
        if self.map.is_valid_qr(q,r) {
          let some_hex = self.map.get_hex_from_qr(q, r);
          if let Some(hex) = some_hex {
            if !hex.is_open() {
              for i in 0..self.appliances.len() {
                let loc = self.appliances[i].get_qr_location();
                if q == loc.x && r == loc.y {
                  // Select appliance
                  found_appliance = true;
                  if self.selected_appliance.is_some() {
                    if i == self.selected_appliance.unwrap() {
                      break;
                    }
                    
                    self.appliances[self.selected_appliance.unwrap()].should_draw_range(false);
                  }
                  
                  self.selected_appliance = Some(i);
                  break;
                }
              }
            }
          }
        }
        
        if !found_appliance {
          if self.selected_appliance.is_some() {
            self.appliances[self.selected_appliance.unwrap()].should_draw_range(false);
          }
          self.selected_appliance = None;
        }
      }
    }
  }
  
  pub fn update_ui(&mut self, delta_time: f32) {
    
  }
  
  pub fn update_placing(&mut self, delta_time: f32) {
    let left_clicked = self.data.left_mouse;
    let right_clicked = self.data.right_mouse;
    let mouse = self.data.mouse_pos;
    let escape_pressed = self.data().keys.escape_pressed();
    
    if right_clicked {
      self.escaped_pressed_last_frame = false;
      self.mouse_state = MouseState::World;
      self.map.unhighlight_all_hexs();
      return;
    }
    
    let mouse_ray = self.camera.mouse_to_world_ray(mouse, self.data.window_dim);
    if mouse_ray.y < 0.0 {
      let mut crnt_pos = self.camera.get_position();
      while crnt_pos.y > 0.0 {
        crnt_pos += mouse_ray;
      }
      crnt_pos -= mouse_ray;
      
      let pix_x = crnt_pos.x;
      let pix_y = crnt_pos.z;
      let clicked_hex = self.map.pixel_to_hex(Vector2::new(pix_x, pix_y));
      let q = clicked_hex.q();
      let r =  clicked_hex.r();
      
      if let Some(appliance) = &mut self.placing_appliance {
        appliance.set_qr_location(q,r, &self.map);
        self.valid_place = self.map.is_valid_qr(q,r);
        if self.selected_appliance.is_some() {
          let some_hex = self.map.get_hex_from_qr(q,r);
          if let Some(hex) = some_hex {
            if !hex.is_highlighted() {
              self.valid_place = false;
            }
          }
        }
      }
      
      if self.placing_appliance.is_some() {
        let mut appliance = self.placing_appliance.clone().unwrap();
        
        if left_clicked {
          self.ray_position = Vector2::new(pix_x, pix_y);
          
          let opt_hex = self.map.get_hex_from_qr(q,r);
          if let Some(hex) = opt_hex {
            if hex.is_open() {
              if self.selected_appliance.is_some() {
                let some_hex = self.map.get_hex_from_qr(q,r);
                if let Some(hex) = some_hex {
                  if !hex.is_highlighted() {
                    self.valid_place = false;
                    return;
                  }
                }
              }
              
              // if moving tower
              if let Some(idx) = self.selected_appliance {
                let qr = self.appliances[idx].get_qr_location();
                self.map.set_hexagon_type(qr.x, qr.y, HexagonType::Open);
                self.appliances.remove(idx);
                self.selected_appliance = Some(self.appliances.len());
                let dist = Hexagon::hex_distance(&Hexagon::new(q,r, "".to_string()), &Hexagon::new(qr.x, qr.y, "".to_string()));
                appliance.moved_tiles(dist);
                self.map.unhighlight_all_hexs();
              } else { 
                if appliance.buy_cost() > self.money {
                  return;
                }
                self.money -= appliance.buy_cost();
              }
              
              appliance.should_draw_range(false);
              self.valid_place = false;
              self.placing_appliance = None;
              self.mouse_state = MouseState::World;
              self.map.set_hexagon_type(q,r,HexagonType::Closed);
              self.appliances.push(appliance);
            }
          }
        }
      }
    } else {
      self.valid_place = false;
    }
  }
  
  pub fn update_neutral(&mut self, real_delta: f32, delta_time: f32) {
    let space_pressed = self.data().keys.space_pressed();
    let scroll_delta = self.data().scroll_delta;
    
    if self.data.window_resized || self.bin >= 100 {
      self.data.next_scene = true;
    }
    
    if self.space_pressed_last_frame && !space_pressed {
      self.space_pressed_last_frame = false;
      match self.game_speed {
        1 => {
          self.game_speed = 2;
        },
        2 => {
          self.game_speed = 4;
        },
        4 => {
          self.game_speed = 8;
        },
        8 => {
          self.game_speed = 16;
        },
        _ => {
          self.game_speed = 1;
        }
      }
    }
    self.space_pressed_last_frame = space_pressed;
    
    if scroll_delta > 0.0 {
      self.camera.process_movement(camera::Direction::Forward, 10.0*real_delta);
    } else if scroll_delta < 0.0 {
      self.camera.process_movement(camera::Direction::Backward, 10.0*real_delta);
    }
    
    let window_dimensions = self.data().window_dim;
    
    self.screen_offset.x=(window_dimensions.x*self.zoom)*0.5;
    self.screen_offset.y=(window_dimensions.y*self.zoom)*0.5;
  }
  
  pub fn update_objects(&mut self, real_delta: f32, delta_time: f32) {
    let delta_steps = (self.total_delta / DELTA_STEP).floor() as usize;
    
    self.map.update(real_delta);
    
    for _ in 0..delta_steps {
      let mut some_food = None;
      if self.map.is_ready() {
        some_food = self.the_food_store.update(DELTA_STEP);
      }
      
      if let Some(food) = some_food {
        self.foods.push(food);
      }
      
      let appliances = &mut self.appliances;
      let foods = &mut self.foods;
      let weapons = &mut self.weapons;
      let m_sizes = &mut self.data.model_sizes;
      let map = &mut self.map;
      let bin = &mut self.bin;
      let money = &mut self.money;
      let selected_appliance = &mut self.selected_appliance;
      
      update_game(map, appliances, foods, weapons, selected_appliance, m_sizes, DELTA_STEP);
      collisions(map, foods, weapons, m_sizes, bin, money, DELTA_STEP);
      
      if self.foods.len() == 0 {
        if self.the_food_store.next_wave() {
          for appliance in &mut self.appliances {
            appliance.decrease_life_expectancy();
          }
        }
      }
      
      self.total_delta -= DELTA_STEP;
    }
  }
  
  pub fn dev_hacks(&mut self, _real_delta: f32, _delta_time: f32) {
    let f1_pressed = self.data.keys.f1_pressed();
    let f2_pressed = self.data.keys.f2_pressed();
    
    if f1_pressed && !self.f1_pressed_last_frame {
      self.foods.clear();
      self.the_food_store.skip_wave();
    }
    
    if f2_pressed && !self.f2_pressed_last_frame {
      self.money += 1000;
    }
    
    
    self.f1_pressed_last_frame = f1_pressed;
    self.f2_pressed_last_frame = f2_pressed;
  }
}

impl Scene for GameScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    if self.data().window_resized {
      Box::new(GameScreen::new_with_data(window_size, self.rng.clone(), self.camera.clone(), self.screen_offset, self.appliances.clone(), self.foods.clone(), self.map.clone(), self.data.model_sizes.clone(), self.weapons.clone(), self.the_food_store.clone(), self.money, self.game_speed, self.bin))
    } else {
      Box::new(MenuScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    let real_delta = delta_time;
    let delta_time = delta_time * self.game_speed as f32;
    self.mut_data().controller.update();
    self.total_delta += delta_time;
    
    match &mut self.mouse_state {
      MouseState::Ui => {
        self.update_ui(delta_time);
      },
      MouseState::World => {
        self.update_world(delta_time);
      },
      MouseState::Placing => {
        self.update_placing(delta_time);
      }
    }
    
    self.update_keypresses(real_delta);
    
    self.update_objects(real_delta, delta_time);
    
    self.update_neutral(real_delta, delta_time);
    
    if !self.map.is_ready() {
      self.placing_appliance = None;
      self.selected_appliance = None;
      self.mouse_state = MouseState::World;
      self.game_speed = 0;
    }
    
    if DEV {
      self.dev_hacks(real_delta, delta_time);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    for food in &self.foods {
      food.draw(draw_calls);
    }
    
    for appliance in &self.appliances {
      let map = &self.map;
      appliance.draw(map, &self.camera, Vector2::new(self.data.window_dim.x as f32, self.data.window_dim.y as f32), draw_calls);
    }
    
    for weapon in &self.weapons {
      weapon.draw(draw_calls);
    }
    
    let cam_pos = self.camera.get_position();
    
    let hexagon_name = "Hexagon".to_string();
    let hexagon_model_size: Vector3<f32> = {
      let mut model_size = Vector3::new(1.0, 1.0, 1.0);
      for model in &self.data().model_sizes {
        if model.0 == hexagon_name {
            model_size = model.1
        }
      }
      
      model_size
    };
    
    self.map.draw(hexagon_model_size, cam_pos.xz(), draw_calls);
    
    let offset = 32.0;
    
    match self.mouse_state {
      MouseState::Placing => {
        if let Some(appliance) = &self.placing_appliance {
          if self.valid_place {
            let map = &self.map;
            let some_hex = self.map.get_hex_from_qr(appliance.get_qr_location().x, appliance.get_qr_location().y);
            if let Some(hex) = some_hex {
              if hex.is_path() || !hex.is_open()|| self.money < appliance.buy_cost() {
                appliance.draw_hologram_invalid(map, draw_calls);
              } else {
                appliance.draw_hologram(map, draw_calls);
              }
            }
          }
        }
      },
      _ => {
        if let Some(idx) = self.selected_appliance {
          let map = &self.map;
          
          self.appliances[idx].draw_range_coloured(map, Vector3::new(0.0, 0.0, 1.0), draw_calls);
          
          let clean_price = self.appliances[idx].clean_cost();
          let sell_price = self.appliances[idx].sell_price();
          
          // UI 
          draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*6.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(0.7, 1.0, 1.0, 1.0), 
                                           "Key t: Cycle targeting priority".to_string(), 
                                           "Arial".to_string()));
          
          draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*5.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(0.7, 1.0, 1.0, 1.0), 
                                           "Key X: Sell appliance $".to_owned() + &(sell_price).to_string(), 
                                           "Arial".to_string()));
          let mut colour = Vector4::new(0.7, 1.0, 1.0, 1.0);
          if clean_price > self.money {
            colour = Vector4::new(1.0, 0.0, 0.0, 1.0);
          }
          draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*4.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "Key C: Cleans appliance $".to_owned() + &(clean_price).to_string(), 
                                           "Arial".to_string()));
          draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*3.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(0.7, 1.0, 1.0, 1.0), 
                                           "Key M: Moves selected appliance".to_string(), 
                                           "Arial".to_string()));
        }
      },
    }
    
    /* 
    ** UI
    */
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(self.data.window_dim.x-264.0, self.data.window_dim.y-offset), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new((self.bin as f32/100.0), 1.0-(self.bin as f32/100.0), 0.0, 1.0), 
                                           "The Bin is ".to_owned() + &(self.bin).to_string() + "% full", 
                                           "Arial".to_string()));
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(self.data.window_dim.x-196.0, self.data.window_dim.y-offset*2.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Money $".to_owned() + &(self.money).to_string(), 
                                           "Arial".to_string()));
    draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x-160.0, self.data.window_dim.y-offset*3.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Key B: Empty Bin $".to_owned() + &(BIN_CLEAN_COST).to_string(), 
                                           "Arial".to_string()));
    draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x*0.5, self.data.window_dim.y-32.0), 
                                           Vector2::new(132.0, 132.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Wave: ".to_owned() + &(self.the_food_store.wave_number() + 1).to_string(), 
                                           "Arial".to_string()));
                                           
    let t_dishwasher = Dishwasher::new(Vector2::new(0,0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), &self.map);
    let t_fridge = Fridge::new(Vector2::new(0,0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), &self.map);
    let t_tenderiser = MeatTenderizer::new(Vector2::new(0,0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), &self.map);
    let t_coffee = CoffeeMachine::new(Vector2::new(0,0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), &self.map);
    let t_salt_grinder = SaltGrinder::new(Vector2::new(0,0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), &self.map);
    
    let dishwasher_cost = t_dishwasher.buy_cost();
    let fridge_cost = t_fridge.buy_cost();
    let tenderiser_cost = t_tenderiser.buy_cost();
    let coffee_cost = t_coffee.buy_cost();
    let salt_grinder_cost = t_salt_grinder.buy_cost();
    
    let mut colour = Vector4::new(1.0, 1.0, 1.0, 1.0);
    if dishwasher_cost > self.money {
      colour = Vector4::new(1.0, 0.0, 0.0, 1.0);
    }
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*2.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "Key 1: Buy Dishwasher $".to_owned() + &(dishwasher_cost).to_string(), 
                                           "Arial".to_string()));
   
   if !self.minimal_ui {
     draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*1.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "  (Range: 3, LE: 2, Single Shot, Medium firing)".to_owned(), 
                                           "Arial".to_string()));
    }
    
    colour = Vector4::new(1.0, 1.0, 1.0, 1.0);
    if fridge_cost > self.money {
      colour = Vector4::new(1.0, 0.0, 0.0, 1.0);
    }
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5+offset*0.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "Key 2: Buy Fridge $".to_owned() + &(fridge_cost).to_string(),
                                           "Arial".to_string()));
    
    if !self.minimal_ui {
      draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*1.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "  (Range: 1, LE: 3, All Hex, Slow firing)".to_string(),
                                           "Arial".to_string()));
    }
    
    colour = Vector4::new(1.0, 1.0, 1.0, 1.0);
    if tenderiser_cost > self.money {
      colour = Vector4::new(1.0, 0.0, 0.0, 1.0);
    }
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*2.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "Key 3: Buy MeatTenderizer $".to_owned() + &(tenderiser_cost).to_string(),
                                           "Arial".to_string()));
    
    if !self.minimal_ui {
      draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*3.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "  (Range: 1, LE: 4, Hex in directions of hex faces, Very Slow firing)".to_string(),
                                           "Arial".to_string()));
    }
    
    colour = Vector4::new(1.0, 1.0, 1.0, 1.0);
    if coffee_cost > self.money {
      colour = Vector4::new(1.0, 0.0, 0.0, 1.0);
    }
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*4.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "Key 4: Buy Coffee Machine $".to_owned() + &(coffee_cost).to_string(),
                                           "Arial".to_string()));
    
    if !self.minimal_ui {
      draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*5.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "  (Range: 2, LE: 5, Buffs: Range up, LE up, AS up, Sell price down)".to_string(),
                                           "Arial".to_string()));
    }
    
    colour = Vector4::new(1.0, 1.0, 1.0, 1.0);
    if salt_grinder_cost > self.money {
      colour = Vector4::new(1.0, 0.0, 0.0, 1.0);
    }
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*6.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "Key 5: Buy Salt Grinder $".to_owned() + &(salt_grinder_cost).to_string(),
                                           "Arial".to_string()));
    
    if !self.minimal_ui {
      draw_calls.push(DrawCall::draw_text_basic(Vector2::new(16.0, self.data.window_dim.y*0.5-offset*7.0), 
                                           Vector2::new(64.0, 64.0), 
                                           colour, 
                                           "  (Range: 2, LE: 3, multi Shot, fast firing)".to_string(),
                                           "Arial".to_string()));
    }
    
    // Game Speed
    
    if !self.minimal_ui {
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x - 196.0, 160.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0),
                                           "Right click to Unselect appliance".to_string(), 
                                           "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(128.0, 80.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0),
                                           "Key v: resets camera".to_string(), 
                                           "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(164.0, 48.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0),
                                           "Key k: resets current map".to_string(), 
                                           "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x - 160.0, 128.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Key F10 for minimal ui".to_string(), 
                                           "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x - 160.0, 96.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Key P to pause".to_string(), 
                                           "Arial".to_string()));
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x - 216.0, 64.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Key Space for 1x/2x/.../16x speed".to_string(), 
                                           "Arial".to_string()));
    }
    
    draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(96.0, 16.0), 
                                           Vector2::new(96.0, 96.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Speed: x".to_owned() + &(self.game_speed).to_string(), 
                                           "Arial".to_string()));
    
    if self.game_speed == 0 && self.map.is_ready() {
      draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(self.data.window_dim.x*0.5, self.data.window_dim.y*0.5),
                                           Vector2::new(196.0, 196.0), 
                                           Vector4::new(1.0, 0.0, 1.0, 1.0), 
                                           "Paused".to_string(), 
                                           "Arial".to_string()));
    }
    
    draw_calls.push(DrawCall::draw_instanced_model("Hexagon".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("BlueHexagon".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("RedHexagon".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("PurpleHexagon".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("GreenHexagon".to_string()));
    
    draw_calls.push(DrawCall::draw_instanced_model("Fridge".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Dishwasher".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("MeatTenderizer".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("CoffeeMachine".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("SaltGrinder".to_string()));
    
    draw_calls.push(DrawCall::draw_instanced_model("Spoon".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Plate".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Salt".to_string()));
    
    draw_calls.push(DrawCall::draw_instanced_model("Strawberry".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Banana".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Cake".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Pineapple".to_string()));
    draw_calls.push(DrawCall::draw_instanced_model("Mushroom".to_string()));
  }
}
