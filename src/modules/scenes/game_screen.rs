use maat_graphics::DrawCall;
use maat_graphics::camera;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::MenuScreen;

use crate::modules::food::Food;
use crate::modules::appliances::{Dishwasher, Fridge};
use crate::modules::appliances::traits::Appliance;
use crate::modules::weapons::{Weapon};
use crate::modules::hexagon::{Layout, Hexagon, HexagonType};
use crate::modules::thefoodstore::FoodStore;

use crate::modules::update::update_game;
use crate::modules::physics::collisions;
use crate::modules::map::Map;

use rand;
use rand::{thread_rng};

use cgmath::{InnerSpace, SquareMatrix, Matrix4, Point3, Deg, Vector2, Vector3, Vector4, PerspectiveFov};

const DEFAULT_ZOOM: f32 = 1.0;
const DELTA_STEP: f32 = 0.01;

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
  screen_offset: Vector2<f32>,
  camera: camera::Camera,
  rng: rand::prelude::ThreadRng,
  last_mouse_pos: Vector2<f32>,
  total_delta: f32,
  map: Map,
  appliances: Vec<Box<Appliance>>,
  foods: Vec<Food>,
  weapons: Vec<Box<Weapon>>,
  ray_position: Vector2<f32>,
  game_speed: i32,
  mouse_state: MouseState,
  bin: i32,
  placing_appliance: Option<Box<Appliance>>,
  valid_place: bool,
  the_food_store: FoodStore,
}

impl GameScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> GameScreen {
    println!("Game Screen");
    
    let mut camera = camera::Camera::default_vk();
    camera.set_position(Vector3::new(83.93359, 128.62776, 55.85842));
    camera.set_pitch(-62.27426);
    camera.set_yaw(210.10083);
    camera.set_move_speed(50.0);
    
    let map = Map::new();
    
    //let tile_pos = map.get_next_path(0) as usize;
   // let square_pos = map.get_path_position(tile_pos);
    
  //  let start_pos = Vector3::new(square_pos.x as f32, 0.0, square_pos.y as f32);
    
   // let position = map.get_tile_position(3, 3);
    let dishwasher = Box::new(Dishwasher::new(Vector2::new(2,2), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, 0.0), &map));
    let dishwasher2 = Box::new(Dishwasher::new(Vector2::new(-2,-3), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, 0.0), &map));
    
    /*
    // Ranges
    let mut hexagons: Vec<Hexagon> = Vec::new();
    
    let radius = 8;
    for q in -radius..radius+1 {
      let r1 = (-radius).max(-q - radius);
      let r2 = radius.min(-q + radius);
      
      for r in r1..r2+1 {
        let dist = Hexagon::hex_distance(Hexagon::new(0, 0, "".to_string()), Hexagon::new(q, r, "".to_string()))%4;
        let mut texture = "Hexagon".to_string();
        match dist {
          0 => { texture = "BlueHexagon".to_string(); },
          1 => { texture = "GreenHexagon".to_string(); },
          2 => { texture = "PurpleHexagon".to_string(); },
          3 => { texture = "RedHexagon".to_string(); },
          _ => {}
        }
        
        hexagons.push(Hexagon::new(q, r, texture.to_string()));
      }
    }*/
    
    let path = map.get_path();
    let food_pos = map.tile_position_from_index(path[0] as usize);
    let tile_loc = map.get_qr_from_index(path[0] as usize);
    let store = FoodStore::new(&map);
    
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      space_pressed_last_frame: false,
      screen_offset: Vector2::new(0.0, 0.0),
      camera: camera,
      rng: thread_rng(),
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      map,
      appliances: vec!(dishwasher, dishwasher2),
      foods: vec!(),
      weapons: Vec::new(),
      ray_position: Vector2::new(0.0, 0.0),
      game_speed: 1,
      mouse_state: MouseState::World,
      bin: 0,
      placing_appliance: None,
      valid_place: false,
      the_food_store: store,
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, screen_offset: Vector2<f32>, appliances: Vec<Box<Appliance>>, foods: Vec<Food>, map: Map, model_sizes: Vec<(String, Vector3<f32>)>, weapons: Vec<Box<Weapon>>, the_food_store: FoodStore, game_speed: i32, bin: i32) -> GameScreen {
    
    GameScreen {
      data: SceneData::new(window_size, model_sizes),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
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
      space_pressed_last_frame: false,
      mouse_state: MouseState::World,
      bin,
      placing_appliance: None,
      valid_place: false,
      the_food_store,
    }
  }
  
  pub fn update_keypresses(&mut self, delta_time: f32) {
    let escape_pressed = self.data().keys.escape_pressed();
    let mouse = self.data.mouse_pos;
    
    let mut one_pressed = self.data.keys.one_pressed();
    let mut two_pressed = self.data.keys.two_pressed();
    let mut w_pressed = self.data.keys.w_pressed();
    let mut a_pressed = self.data.keys.a_pressed();
    let mut s_pressed = self.data.keys.s_pressed();
    let mut d_pressed = self.data.keys.d_pressed();
    let r_pressed = self.data().keys.r_pressed();
    let f_pressed = self.data().keys.f_pressed();
    let p_pressed = self.data().keys.p_pressed();
    
    
    if self.escaped_pressed_last_frame && !escape_pressed {
      self.escaped_pressed_last_frame = false;
      self.mut_data().next_scene = true;
    }
    
    self.escaped_pressed_last_frame = escape_pressed;
    
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
    
    if p_pressed {
      self.game_speed = 0;
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
      self.placing_appliance = Some(Box::new(Dishwasher::new(Vector2::new(0,0), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, 0.0), &self.map)));
      if let Some(appliance) = &mut self.placing_appliance {
        let foods = &mut self.foods;
        let weapons = &mut self.weapons;
        let m_sizes = &mut self.data.model_sizes;
        let map = &self.map;
        
        appliance.update(foods, weapons, m_sizes, map, 0.0);
      }
      self.mouse_state = MouseState::Placing;
    }
    if two_pressed {
      self.placing_appliance = Some(Box::new(Fridge::new(Vector2::new(0,0), Vector3::new(3.0, 3.0, 3.0), Vector3::new(0.0, 0.0, 0.0), &self.map)));
      if let Some(appliance) = &mut self.placing_appliance {
        let foods = &mut self.foods;
        let weapons = &mut self.weapons;
        let m_sizes = &mut self.data.model_sizes;
        let map = &self.map;
        
        appliance.update(foods, weapons, m_sizes, map, 0.0);
      }
      self.mouse_state = MouseState::Placing;
    }
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
    let mouse = self.data.mouse_pos;
    
    if left_clicked {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        let x_offset = self.last_mouse_pos.x - mouse.x;
        let y_offset = mouse.y - self.last_mouse_pos.y;
        self.camera.process_mouse_movement(x_offset, y_offset);
      }
    }
  }
  
  pub fn update_ui(&mut self, delta_time: f32) {
    
  }
  
  pub fn update_placing(&mut self, delta_time: f32) {
    let left_clicked = self.data.left_mouse;
    let mouse = self.data.mouse_pos;
    
    let mouse_ray = self.camera.mouse_to_world_ray(mouse, self.data.window_dim);
    if mouse_ray.y < 0.0 {
      let mut crnt_pos = self.camera.get_position();
      while crnt_pos.y > 0.0 {
        crnt_pos += mouse_ray;
      }
      crnt_pos -= mouse_ray;
      
      let pix_x = crnt_pos.x;
      let pix_y = crnt_pos.z;
      let clicked_hex = self.map.pixel_to_hex(pix_x, pix_y);
      let q = clicked_hex.q();
      let r =  clicked_hex.r();
      
      if let Some(appliance) = &mut self.placing_appliance {
        appliance.set_qr_location(q,r, &self.map);
        if let Some(hex) = self.map.get_hex_from_qr(q,r) {
          if !hex.is_path() {
            self.valid_place = true;
          } else {
            self.valid_place = false;
          }
        } else {
          self.valid_place = false;
        }
      }
      
      if self.placing_appliance.is_some() {
        let appliance = self.placing_appliance.clone().unwrap();
        
        if left_clicked {
          self.ray_position = Vector2::new(pix_x, pix_y);
          
          let opt_hex = self.map.get_hex_from_qr(q,r);
          if let Some(hex) = opt_hex {
            if hex.is_open() {
              self.appliances.push(appliance);
              self.map.set_hexagon_type(q,r,HexagonType::Closed);
              self.valid_place = false;
              self.placing_appliance = None;
              self.mouse_state = MouseState::World;
            }
          }
        }
      }
    } else {
      self.valid_place = false;
    }
  }
  
  pub fn update_neutral(&mut self, delta_time: f32) {
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
        16 => {
          self.game_speed = 32;
        },
        32 => {
          self.game_speed = 64;
        },
        _ => {
          self.game_speed = 1;
        }
      }
    }
    self.space_pressed_last_frame = space_pressed;
    
    if scroll_delta > 0.0 {
      self.camera.process_movement(camera::Direction::Forward, DELTA_STEP);
    } else if scroll_delta < 0.0 {
      self.camera.process_movement(camera::Direction::Backward, DELTA_STEP);
    }
    
    let window_dimensions = self.data().window_dim;
    
    self.screen_offset.x=(window_dimensions.x*self.zoom)*0.5;
    self.screen_offset.y=(window_dimensions.y*self.zoom)*0.5;
  }
  
  pub fn update_objects(&mut self, delta_time: f32) {
    let delta_steps = (self.total_delta / DELTA_STEP).floor() as usize;
    for _ in 0..delta_steps {
      let some_food = self.the_food_store.update(DELTA_STEP);
      if let Some(food) = some_food {
        self.foods.push(food);
      }
      
      // println!("Update is happening: {}", self.total_delta);
      let appliances = &mut self.appliances;
      let foods = &mut self.foods;
      let weapons = &mut self.weapons;
      let m_sizes = &mut self.data.model_sizes;
      let map = &self.map;
      let bin = &mut self.bin;
      
      update_game(map, appliances, foods, weapons, m_sizes, DELTA_STEP);
      collisions(map, foods, weapons, m_sizes, bin, DELTA_STEP);
      
      if self.foods.len() == 0 {
        self.the_food_store.next_wave();
      }
      
      self.total_delta -= DELTA_STEP;
    }
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
      Box::new(GameScreen::new_with_data(window_size, self.rng.clone(), self.camera.clone(), self.screen_offset, self.appliances.clone(), self.foods.clone(), self.map.clone(), self.data.model_sizes.clone(), self.weapons.clone(), self.the_food_store.clone(), self.game_speed, self.bin))
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
    
    self.update_objects(delta_time);
    
    self.update_neutral(delta_time);
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::set_texture_scale(DEFAULT_ZOOM));
    draw_calls.push(DrawCall::lerp_ortho_camera_to_pos(self.screen_offset, Vector2::new(0.05, 0.05)));
    draw_calls.push(DrawCall::lerp_ortho_camera_to_size(self.data.window_dim*self.zoom, Vector2::new(0.05, 0.05)));
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    for food in &self.foods {
      food.draw(draw_calls);
    }
    
    for appliance in &self.appliances {
      appliance.draw(draw_calls);
    }
    
    for weapon in &self.weapons {
      weapon.draw(draw_calls);
    }
    
    self.map.draw(draw_calls);
    
    match self.mouse_state {
      MouseState::Placing => {
        if let Some(appliance) = &self.placing_appliance {
          if self.valid_place {
            appliance.draw_hologram(draw_calls);
            
            let mut hexagons: Vec<Hexagon> = Vec::new();
            
            let radius = appliance.get_range() as i32;
            for q in -radius..radius+1 {
              let r1 = (-radius).max(-q - radius);
              let r2 = radius.min(-q + radius);
              
              for r in r1..r2+1 {
                let dist = Hexagon::hex_distance(Hexagon::new(0, 0, "".to_string()), Hexagon::new(q, r, "".to_string()))%4;
                let mut texture = "PurpleHexagon".to_string();
                
                hexagons.push(Hexagon::new(q, r, texture.to_string()));
              }
            }
            
            let pos = appliance.get_position();
            let origin = Vector2::new(pos.x, pos.z);
            let layout = Layout::new(origin, Vector2::new(8.0,8.0));
            
            for hexagon in hexagons {
              let location = layout.hex_to_pixel(hexagon.clone());
         
              let position = Vector3::new(location.x, 0.1, location.y);
              let height = 1.2;
              draw_calls.push(DrawCall::draw_hologram_model(position,
                                                   Vector3::new(8.0/4.0, height, 8.0/4.0),
                                                   Vector3::new(0.0, 90.0, 0.0), 
                                                   hexagon.get_model()));
            }
          }
        }
      },
      _ => {},
    }
    
    draw_calls.push(DrawCall::draw_model(Vector3::new(self.ray_position.x, 0.0, self.ray_position.y), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, 0.0), "Chair".to_string()));
    
    /* 
    ** UI
    */
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(64.0, self.data.window_dim.y-96.0), 
                                           Vector2::new(128.0, 128.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Bin is ".to_owned() + &(self.bin).to_string() + "% full.", 
                                           "Arial".to_string()));
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(64.0, self.data.window_dim.y-128.0), 
                                           Vector2::new(128.0, 128.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Wave: ".to_owned() + &(self.the_food_store.wave_number()).to_string(), 
                                           "Arial".to_string()));
                                           
    // Game Speed
    draw_calls.push(DrawCall::draw_text_basic_centered(Vector2::new(64.0, 16.0), 
                                           Vector2::new(196.0, 196.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "Speed: x".to_owned() + &(self.game_speed).to_string(), 
                                           "Arial".to_string()));
    
  }
}
