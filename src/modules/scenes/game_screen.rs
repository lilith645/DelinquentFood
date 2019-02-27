use maat_graphics::DrawCall;
use maat_graphics::math;
use maat_graphics::camera;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::MenuScreen;

use crate::modules::hexagon::Hexagon;
use crate::modules::hexagon::Layout;

use crate::modules::food::Food;
use crate::modules::towers::Fridge;
use crate::modules::towers::Dishwasher;
use crate::modules::towers::traits::Tower;

use crate::modules::update::update_game;
use crate::modules::physics::collisions;
use crate::modules::map::Map;

use rand;
use rand::{thread_rng};

use cgmath::{Rad, Deg, PerspectiveFov, SquareMatrix, Matrix4, Vector2, Vector3, Vector4, Point3};

const DEFAULT_ZOOM: f32 = 1.0;
const DELTA_STEP: f32 = 0.01;

pub struct GameScreen {
  data: SceneData,
  zoom: f32,
  escaped_pressed_last_frame: bool,
  screen_offset: Vector2<f32>,
  camera: camera::Camera,
  rng: rand::prelude::ThreadRng,
  last_mouse_pos: Vector2<f32>,
  total_delta: f32,
  map: Map,
  towers: Vec<Box<Tower>>,
  foods: Vec<Food>,
}

impl GameScreen {
  pub fn new(window_size: Vector2<f32>) -> GameScreen {
    println!("Game Screen");
    
    let mut camera = camera::Camera::default_vk();
    camera.set_position(Vector3::new(83.93359, 128.62776, 55.85842));
    camera.set_pitch(-62.27426);
    camera.set_yaw(210.10083);
    
    let map = Map::new();
    
    //let tile_pos = map.get_next_path(0) as usize;
   // let square_pos = map.get_path_position(tile_pos);
    
  //  let start_pos = Vector3::new(square_pos.x as f32, 0.0, square_pos.y as f32);
    
   // let position = map.get_tile_position(3, 3);
    let fridge = Box::new(Dishwasher::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0), Vector2::new(3,3)));
    
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
    
    let mut path = map.get_path();
    let food_pos = map.tile_position_from_index(path[0] as usize);
    let tile_loc = map.get_qr_from_index(path[0] as usize);
    
    GameScreen {
      data: SceneData::new(window_size),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      screen_offset: Vector2::new(0.0, 0.0),
      camera: camera,
      rng: thread_rng(),
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      map,
      towers: vec!(fridge),
      foods: vec!(Food::new(Vector3::new(food_pos.x, 0.0, food_pos.y), "Strawberry".to_string(), path, tile_loc)),
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, screen_offset: Vector2<f32>, towers: Vec<Box<Tower>>, foods: Vec<Food>, map: Map) -> GameScreen {
    
    GameScreen {
      data: SceneData::new(window_size),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      screen_offset,
      camera,
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
      map,
      towers,
      foods,
    }
  }
  
  pub fn update_keypresses(&mut self, escape_pressed: bool) {
    if self.escaped_pressed_last_frame && !escape_pressed {
      self.escaped_pressed_last_frame = false;
      self.mut_data().next_scene = true;
    }
    
    self.escaped_pressed_last_frame = escape_pressed;
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
      Box::new(GameScreen::new_with_data(window_size, self.rng.clone(), self.camera.clone(), self.screen_offset, self.towers.clone(), self.foods.clone(), self.map.clone()))
    } else {
      Box::new(MenuScreen::new(window_size))
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    self.mut_data().controller.update();
    self.total_delta += delta_time;
    
    let mouse = self.data().mouse_pos;
    let left_clicked = self.data().left_mouse;
    let _left_mouse_dragged = self.data().left_mouse_dragged;
    let _middle_mouse_dragged = self.data().middle_mouse_dragged;
    let _right_mouse_dragged = self.data().right_mouse_dragged;
    let escape_pressed = self.data().keys.escape_pressed();
    let _scroll_delta = self.data().scroll_delta;
    let _keys_pressed_this_frame = self.get_keys_pressed_this_frame();
    let mut w_pressed = self.data().keys.w_pressed();
    let mut a_pressed = self.data().keys.a_pressed();
    let mut s_pressed = self.data().keys.s_pressed();
    let mut d_pressed = self.data().keys.d_pressed();
    let mut r_pressed = self.data().keys.r_pressed();
    let mut f_pressed = self.data().keys.f_pressed();
    
    if left_clicked {
   /*   let fov = 60.0;
      let aspect = self.data().window_dim.x / self.data().window_dim.y;
      let near = 0.1;
      let far = 256.0;
     // let f = (math::to_radians(fov) / 2.0).cot();
      let perspective = PerspectiveFov {
        fovy: Deg(fov).into(),
        aspect,
        near,
        far,
      };
      let perspective = perspective.to_perspective();
      
      let (c_pos, c_center, c_up) = self.camera.get_look_at();
      
      let view = Matrix4::look_at(Point3::new(c_pos.x, c_pos.y, c_pos.z), 
                                  Point3::new(c_center.x, c_center.y, c_center.z), c_up);
      
      let invt_view = view.invert().unwrap();
      let invt_perspective = Matrix4::from(perspective).invert().unwrap();
      
      let position = self.camera.get_position();
      let position = Vector4::new(position.x, position.y, position.z, 1.0);
      let pos = invt_perspective * invt_view * position;
      println!("Position: {:?}", pos);
      let pix_x = position.x;
      let pix_y = position.y;
      let clicked_hex = self.map.pixel_to_hex(pix_x, pix_y);
      
      self.map.highlight_hex(clicked_hex);*/
      let mut temp_camera = self.camera.clone();
      let mut position = self.camera.get_position();
      
      if mouse.y > self.data().window_dim.y*0.5 {
        position.y += mouse.y - self.data().window_dim.y*0.5;
      } else if mouse.y < self.data().window_dim.y*0.5 {
        position.y -= self.data().window_dim.y*0.5-mouse.y;
      }
      
      if mouse.x > self.data().window_dim.x*0.5 {
        position.x += mouse.x - self.data().window_dim.x*0.5;
      } else if mouse.x < self.data().window_dim.x*0.5 {
        position.x -= self.data().window_dim.x*0.5-mouse.x;
      }
      
      let front = temp_camera.get_front();
      
      while position.y > 0.0 {
        position += front;
      }
      
      let pix_x = position.x;
      let pix_y = position.z;
      let clicked_hex = self.map.pixel_to_hex(pix_x, pix_y);
      
      self.map.highlight_hex(clicked_hex);
      
      println!("Pixel location: {}, {}", pix_x, pix_y);
    }
    
    if self.data().window_resized {
      self.mut_data().next_scene = true;
    }
    
    self.update_keypresses(escape_pressed);
    match self.update_controller_input() {
      (w, a, s, d, x_offset, y_offset) => {
        if w { w_pressed = w; }
        if a { a_pressed = a; }
        if s { s_pressed = s; }
        if d { d_pressed = d; }
        self.camera.process_mouse_movement(x_offset, y_offset);
      }
    }
    
    if left_clicked {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        let x_offset = self.last_mouse_pos.x - mouse.x;
        let y_offset = mouse.y - self.last_mouse_pos.y;
        self.camera.process_mouse_movement(x_offset, y_offset);
      }
    }
    self.last_mouse_pos = mouse;
    
    if w_pressed {
      self.camera.process_movement(camera::Direction::PositiveX, delta_time);
    }
    if a_pressed {
      self.camera.process_movement(camera::Direction::PositiveZ, delta_time);
    }
    if s_pressed {
      self.camera.process_movement(camera::Direction::NegativeX, delta_time);
    }
    if d_pressed {
      self.camera.process_movement(camera::Direction::NegativeZ, delta_time);
    }
    if r_pressed {
      self.camera.process_movement(camera::Direction::PositiveY, delta_time);
    }
    if f_pressed {
      self.camera.process_movement(camera::Direction::NegativeY, delta_time);
    }
    
    let delta_steps = (self.total_delta / DELTA_STEP).floor() as usize;
    
    for i in 0..delta_steps {
     // println!("Update is happening: {}", self.total_delta);
      update_game(&self.map, &mut self.towers, &mut self.foods, DELTA_STEP);
      collisions(DELTA_STEP);
      
      self.total_delta -= DELTA_STEP;
    }
    
    let window_dimensions = self.data().window_dim;
    
    self.screen_offset.x=(window_dimensions.x*self.zoom)*0.5;
    self.screen_offset.y=(window_dimensions.y*self.zoom)*0.5;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::set_texture_scale(DEFAULT_ZOOM));
    draw_calls.push(DrawCall::lerp_ortho_camera_to_pos(self.screen_offset, Vector2::new(0.05, 0.05)));
    draw_calls.push(DrawCall::lerp_ortho_camera_to_size(self.data.window_dim*self.zoom, Vector2::new(0.05, 0.05)));
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    let pos = self.map.get_tile_position(0, -1);
    draw_calls.push(DrawCall::draw_model(Vector3::new(pos.x, 3.6, pos.y), Vector3::new(2.0, 2.0, 2.0), Vector3::new(0.0, 0.0, 0.0), "Tower".to_string()));
    
    for food in &self.foods {
      food.draw(draw_calls);
    }
    
    for tower in &self.towers {
      tower.draw(draw_calls);
    }
    
    self.map.draw(draw_calls);
  }
}
