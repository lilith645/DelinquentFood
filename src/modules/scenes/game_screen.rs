use maat_graphics::DrawCall;
use maat_graphics::math;
use maat_graphics::camera;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::MenuScreen;

use crate::modules::update::update_game;
use crate::modules::physics::collisions;

use rand;
use rand::{thread_rng};

use cgmath::{Vector2, Vector3};

const DEFAULT_ZOOM: f32 = 1.0;
const DELTA_STEP: f32 = 0.1;

pub struct GameScreen {
  data: SceneData,
  zoom: f32,
  escaped_pressed_last_frame: bool,
  screen_offset: Vector2<f32>,
  camera: camera::Camera,
  rng: rand::prelude::ThreadRng,
  last_mouse_pos: Vector2<f32>,
  total_delta: f32,
}

impl GameScreen {
  pub fn new(window_size: Vector2<f32>) -> GameScreen {
    println!("Game Screen");
    
    let mut camera = camera::Camera::default_vk();
    camera.set_position(Vector3::new(-15.0, 30.0, 0.0));
    camera.set_pitch(-60.0);
    
    GameScreen {
      data: SceneData::new(window_size),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      screen_offset: Vector2::new(0.0, 0.0),
      camera: camera,
      rng: thread_rng(),
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, screen_offset: Vector2<f32>) -> GameScreen {
    GameScreen {
      data: SceneData::new(window_size),
      zoom: 1.0, // 0.5 to 2.0
      escaped_pressed_last_frame: false,
      screen_offset,
      camera,
      rng,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      total_delta: 0.0,
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
      Box::new(GameScreen::new_with_data(window_size, self.rng.clone(), self.camera.clone(), self.screen_offset))
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
    
    let delta_steps = (self.total_delta / DELTA_STEP).floor() as usize;
    
    for i in 0..delta_steps {
      update_game(DELTA_STEP);
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
    
        draw_calls.push(DrawCall::draw_model(Vector3::new(-4.0, 1.2, -0.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(-90.0, 0.0, 0.0), "Lance".to_string()));
    draw_calls.push(DrawCall::draw_model(Vector3::new(-4.0, 5.0, -7.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0), "Chair".to_string()));
    draw_calls.push(DrawCall::draw_model(Vector3::new(5.0, 1.8, -5.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0), "Tower".to_string()));
    draw_calls.push(DrawCall::draw_model(Vector3::new(0.0, 0.0, 0.0), Vector3::new(10.0, 1.0, 10.0), Vector3::new(0.0, 0.0, 0.0), "Floor".to_string()));
  }
}
