use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::GameScreen;

use crate::modules::system_interface::MainMenuUserInterface;

use cgmath::Vector2;

pub struct MenuScreen {
  data: SceneData,
  ui: MainMenuUserInterface,
}

impl MenuScreen {
  pub fn new(window_size: Vector2<f32>) -> MenuScreen {
    println!("Menu Screen");
    
    MenuScreen {
      data: SceneData::new(window_size),
      ui: MainMenuUserInterface::new(window_size),
    }
  }
}

impl Scene for MenuScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    if self.data().window_resized {
      Box::new(MenuScreen::new(window_size))
    } else {
      Box::new(GameScreen::new(window_size))
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    
    let mouse = self.data().mouse_pos;
    let left_clicked = self.data().left_mouse;
    let scroll_delta = self.data().scroll_delta;
    let keys_pressed_this_frame = self.get_keys_pressed_this_frame();
    
    self.ui.update(delta_time, mouse, left_clicked, &keys_pressed_this_frame, scroll_delta);
    
    if self.ui.start_button_pressed() || self.data().window_resized {
      self.mut_data().next_scene = true;
    }
    
    if self.ui.options_button_pressed() {
      println!("optins button pressed");
      self.ui.show_options_menu();
    }
    
    if self.ui.exit_button_pressed() {
      self.mut_data().should_close = true;
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let _dim = self.data().window_dim;
    let (_width, _height) = (_dim.x as f32, _dim.y as f32);
    self.ui.draw(draw_calls);
    draw_calls.push(DrawCall::reset_ortho_camera());
  }
}
