use maat_graphics::DrawCall;

use crate::modules::system_interface::Button;

use cgmath::Vector2;
use cgmath::Vector4;

#[derive(Clone, PartialEq)]
enum Direction {
  _Up,
  _Down,
  _Right,
  _Left,
}

#[derive(Clone, PartialEq)]
pub struct Selection {
  _position: Vector2<f32>,
  _size: Vector2<f32>,
  _direction: Direction,
  buttons: Vec<Button>,
}

impl Selection {
  fn _new_empty(position: Vector2<f32>, size: Vector2<f32>, _direction: Direction, buttons: Vec<Button>) -> Selection {
    Selection {
      _position: position,
      _size: size,
      _direction: Direction::_Up,
      buttons: buttons,
    }
  }
  
  pub fn _new_upwards(position: Vector2<f32>, size: Vector2<f32>, relative_position: Vector2<f32>, 
                     text_size: Vector2<f32>, text_colour: Vector4<f32>, spacing: f32, 
                     option_names: Vec<String>, option_colours: Vec<Vector4<f32>>, selected_background: Vector4<f32>, 
                     center_text: bool, text: Vec<String>, font: String) -> Selection {
    let mut buttons = Vec::with_capacity(option_names.len());
    
    for i in 0..option_names.len() {
      let num = i as f32;
      let position = {
        let temp_pos = position;
        let offset;
        offset = Vector2::new(0.0, size.y*num + spacing*num);
        (temp_pos + offset)
      };
      
      let option_colour = option_colours[i].clone();
      let mut pressed_colour = option_colour +option_colour*0.10;
      if pressed_colour.x > 1.0 {
        pressed_colour.x = 1.0;
      }
      if pressed_colour.y > 1.0 {
        pressed_colour.y = 1.0;
      }
      if pressed_colour.z > 1.0 {
        pressed_colour.z = 1.0;
      }
      if pressed_colour.w > 1.0 {
        pressed_colour.w = 1.0;
      }
      
      buttons.push(Button::new_button(option_names[i].clone(), position, size, relative_position, text_size, 
                    text_colour, pressed_colour, option_colour, true, center_text, text[i].clone(), font.clone())
                    ._toggle_background(size+Vector2::new(5.0,5.0), selected_background));
    }
    
    buttons[0].toggle_on();
    Selection::_new_empty(position, size, Direction::_Up, buttons)
  }
  
  pub fn _new_upwards_textured(position: Vector2<f32>, size: Vector2<f32>, spacing: f32, option_names: Vec<String>, option_textures: Vec<String>, selected_background: Vector4<f32>) -> Selection {
    let mut buttons = Vec::with_capacity(option_names.len());
    
    for i in 0..option_names.len() {
      let num = i as f32;
      let position = {
        let temp_pos = position;
        let offset;
        offset = Vector2::new(0.0, size.y*num + spacing*num);
        (temp_pos + offset)
      };
      
      buttons.push(Button::new_textured_button(option_names[i].clone(), position, size, option_textures[i].clone(), option_textures[i].clone(), true)._toggle_background(size+Vector2::new(5.0,5.0), selected_background));
    }
    
    buttons[0].toggle_on();
    Selection::_new_empty(position, size, Direction::_Up, buttons)
  }
  /*
  pub fn _new_left(position: Vector2<f32>, size: Vector2<f32>, spacing: f32, options: Vec<String>) -> Selection {
    Selection {
      direction: Direction::_Left,
      .. Selection::new_empty(position, size, spacing, options)
    }
  }
  
  pub fn _new_right(position: Vector2<f32>, size: Vector2<f32>, spacing: f32, options: Vec<String>) -> Selection {
    Selection {
      direction: Direction::_Right,
      .. Selection::new_empty(position, size, spacing, options)
    }
  }
  
  pub fn _new_downwards(position: Vector2<f32>, size: Vector2<f32>, spacing: f32, options: Vec<String>) -> Selection {
    Selection {
      direction: Direction::_Down,
      .. Selection::new_empty(position, size, spacing, options)
    }
  }*/
  
  pub fn _is_touching_button(&self, index: usize, against: Vector2<f32>) -> bool {
    if index >= self.buttons.len() {
      return false;
    }
    
    self.buttons[index].is_touching(against)
  }
  
  pub fn get_selected_option(&self) -> Option<String> {
    let mut selection = None;
    for i in 0..self.buttons.len() {
      if self.buttons[i].is_toggled() {
        selection = Some(self.buttons[i].get_name());
        break;
      }
    }
    selection
  }
  
  pub fn update(&mut self, _delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool) {
    if left_mouse {
      for i in 0..self.buttons.len() {
        if self.buttons[i].is_touching(mouse_pos) {
          if let Some(name) = self.get_selected_option() {
            for button in &mut self.buttons {
              if button.name_matches(&name) {
                button.toggle_off();
              }
            }
          }
          
          self.buttons[i].toggle_on();
          break;
        }
      }
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for button in &self.buttons {
      button.draw(draw_calls);
    }
  }
}
