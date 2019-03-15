use maat_graphics::DrawCall;

use crate::modules::system_interface::Widget;
use crate::modules::system_interface::OptionsUi;

use cgmath::Vector2;
use cgmath::Vector4;

//const BACKGROUND_INDEX: usize = 0;
const MENU_OPTIONS_INDEX: usize = 1;

const MENU_OPTIONS_NAME: &str = "MenuOptions";
const BACKGROUND_NAME: &str = "Background";
const START_GAME_NAME: &str = "StartGame";
const EASY_GAME_NAME: &str = "EasyGame";
const MEDIUM_GAME_NAME: &str = "MediumGame";
const HARD_GAME_NAME: &str = "HardGame";
const OPTION_BUTTON_NAME: &str = "Options";
const EXIT_BUTTON_NAME: &str = "ExitGameButton";

pub struct UserInterface {
  options_menu: OptionsUi,
  widgets: Vec<Widget>,
}

impl UserInterface {
  pub fn new(window_size: Vector2<f32>) -> UserInterface {
    let mut widgets: Vec<Widget> = Vec::new();
    
    let menu_width = 512.0;
    
    let button_location = 192.0;
    
    let button_width = 128.0;
    let button_height =32.0;
    let button_offset = button_height + button_height*0.5;
    
    let text_colour = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let button_colour = Vector4::new(0.5019, 0.749, 1.0, 1.0);
    let background_colour = Vector4::new(0.0, 0.1411, 0.4, 1.0);
    
    widgets.push(
      Widget::new(BACKGROUND_NAME.to_string(), Vector2::new(window_size.x*0.5, window_size.y*0.5), 
                   Vector2::new(window_size.x, window_size.y),
                   background_colour)
    );
    
    widgets.push(
      Widget::new(MENU_OPTIONS_NAME.to_string(), Vector2::new(window_size.x*0.5, window_size.y*0.5), 
                  Vector2::new(menu_width, 256.0),
                  background_colour)
                .with_button(Vector2::new(menu_width*0.5-button_width*1.2, button_location),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             EASY_GAME_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(1.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Easy".to_string(), "Arial".to_string())
                .with_button(Vector2::new(menu_width*0.5, button_location),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             MEDIUM_GAME_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(1.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Medium".to_string(), "Arial".to_string())
                .with_button(Vector2::new(menu_width*0.5+button_width*1.2, button_location),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             HARD_GAME_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(1.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Hard".to_string(), "Arial".to_string())
                .with_button(Vector2::new(menu_width*0.5, button_location-button_offset*1.0),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             OPTION_BUTTON_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(0.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Options".to_string(), "Arial".to_string())
                .with_button(Vector2::new(menu_width*0.5, button_location-button_offset*2.0),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             EXIT_BUTTON_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(0.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Exit".to_string(), "Arial".to_string())
    );
    
    let options = OptionsUi::new(window_size);
    
    UserInterface {
      options_menu: options,
      widgets: widgets,
    }
  }
  
  pub fn _is_touching(&self, at_location: Vector2<f32>) -> bool {
    let mut touched = false;
    for widget in &self.widgets {
      if widget._is_hidden() {
        continue;
      }
      
      if widget.is_touching(at_location) {
        touched = true;
        break;
      }
    }
    
    touched
  }
  
  pub fn easy_button_pressed(&self) -> bool {
    self.widgets[MENU_OPTIONS_INDEX].get_button_state(&EASY_GAME_NAME.to_string())
  }
  
  pub fn medium_button_pressed(&self) -> bool {
    self.widgets[MENU_OPTIONS_INDEX].get_button_state(&MEDIUM_GAME_NAME.to_string())
  }
  
  pub fn hard_button_pressed(&self) -> bool {
    self.widgets[MENU_OPTIONS_INDEX].get_button_state(&HARD_GAME_NAME.to_string())
  }
  
  pub fn start_button_pressed(&self) -> bool {
    self.widgets[MENU_OPTIONS_INDEX].get_button_state(&START_GAME_NAME.to_string())
  }
  
  pub fn options_button_pressed(&self) -> bool {
    self.widgets[MENU_OPTIONS_INDEX].get_button_state(&OPTION_BUTTON_NAME.to_string())
  }
  
  pub fn exit_button_pressed(&self) -> bool {
    self.widgets[MENU_OPTIONS_INDEX].get_button_state(&EXIT_BUTTON_NAME.to_string())
  }
  
  pub fn show_options_menu(&mut self) {
    self.options_menu.show();
  }
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, keys_pressed_this_frame: &Vec<String>, scroll_delta: f32) {
      for widget in &mut self.widgets {
        if widget.is_touching(mouse_pos) {
          let mut modified_left_mouse = left_mouse;
          if !self.options_menu.is_hidden() {
            modified_left_mouse = false;
          }
          widget.update(delta_time, mouse_pos, modified_left_mouse, keys_pressed_this_frame, scroll_delta);
        }
      }
    
    self.options_menu.update(delta_time, mouse_pos, left_mouse, keys_pressed_this_frame, scroll_delta);
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for widget in &self.widgets {
      widget.draw(draw_calls);
    }
    self.options_menu.draw(draw_calls);
  }
}
