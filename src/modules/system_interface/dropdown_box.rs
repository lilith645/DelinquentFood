use maat_graphics::DrawCall;

use crate::modules::system_interface::Button;
use crate::modules::system_interface::Slider;

use cgmath::Vector2;
use cgmath::Vector4;

#[derive(Clone)]
pub struct DropdownBox{
  name: String,
  position: Vector2<f32>,
  dropdown_size: f32,
  button_size: Vector2<f32>,
  chosen_option: usize,
  selected_button: Button,
  buttons: Vec<Button>,
  is_dropped_down: bool,
  scroll_bar: Option<Slider>,
  dynamic: bool,
}

impl DropdownBox {
   pub fn _new(name: String, position: Vector2<f32>, size: Vector2<f32>,  dropdown_size: f32, scroll_speed: f32,
             button_text_size: Vector2<f32>,  button_colour: Vector4<f32>, scroll_bar_width: f32, 
             scroll_bar_primay_colour: Vector4<f32>, scroll_bar_secondary_colour: Vector4<f32>,
             selected_name: String, selected_display_text: String, button_names: Vec<String>, button_text: Vec<String>, font: String) -> DropdownBox {
    debug_assert!(position.x > 0.0, "Position x must be within window boundries");
    debug_assert!(position.y > 0.0, "Position y must be within window boundries");
    debug_assert!(dropdown_size > 1.0, "Drop down size needs to be 1 or larger");
    debug_assert!(button_names.len() > 0, "Drop down box requires at least 1 option at creation");
    debug_assert!(button_names.len() == button_text.len(), "Drop down box requires text for every button name\n Button names length: {} != button text length: {}", button_names.len(), button_text.len());
    
    let mut buttons = Vec::with_capacity(button_names.len());
    
    let offset = size.y;
    
    for i in 0..button_names.len() {
      let pos_y = position.y-offset*(i+1) as f32;
      let button = Button::new_button(button_names[i].to_string(), Vector2::new(position.x, pos_y), size, 
                   Vector2::new(size.x*0.5, size.y*0.15), button_text_size, 
                   Vector4::new(0.0, 0.0, 0.0, 1.0), button_colour-Vector4::new(0.2, 0.2, 0.2, 0.0), button_colour, 
                   false, true, button_text[i].to_string(), font.to_string());
      buttons.push(button);
    }
    
    let selected_button = Button::new_button(selected_name.to_string(), position, size, 
                          Vector2::new(size.x*0.5, size.y*0.2), button_text_size, 
                          Vector4::new(0.0, 0.0, 0.0, 1.0), button_colour-Vector4::new(0.2, 0.2, 0.2, 0.0), button_colour, 
                          false, true, selected_display_text.to_string(), font.to_string());
    
    let mut slider = None; 
    
    if size.y*button_names.len() as f32 > dropdown_size {
      slider = Some(Slider::_new("Scrollbar".to_string(), scroll_bar_primay_colour, scroll_bar_secondary_colour, Vector2::new(position.x+size.x*0.5+scroll_bar_width*0.5, position.y+size.y*0.5-dropdown_size*0.5), Vector2::new(scroll_bar_width, dropdown_size-size.y*0.5), Vector2::new(scroll_bar_width*0.75, 15.0), 0.0, scroll_speed, true));
    }
    DropdownBox {
      name: name,
      position: position,
      dropdown_size: dropdown_size,
      button_size: size,
      chosen_option: 0,
      selected_button: selected_button,
      buttons: buttons,
      is_dropped_down: false,
      scroll_bar: slider,
      dynamic: false,
    }
  }
  
  pub fn _new_dynamic(name: String, position: Vector2<f32>, size: Vector2<f32>,  dropdown_size: f32, scroll_speed: f32,
             button_text_size: Vector2<f32>,  button_colour: Vector4<f32>, scroll_bar_width: f32, 
             scroll_bar_primay_colour: Vector4<f32>, scroll_bar_secondary_colour: Vector4<f32>,
             button_names: Vec<String>, button_text: Vec<String>, font: String) -> DropdownBox {
    debug_assert!(position.x > 0.0, "Position x must be within window boundries");
    debug_assert!(position.y > 0.0, "Position y must be within window boundries");
    debug_assert!(dropdown_size > 1.0, "Drop down size needs to be 1 or larger");
    debug_assert!(button_names.len() > 0, "Drop down box requires at least 1 option at creation");
    debug_assert!(button_names.len() == button_text.len(), "Drop down box requires text for every button name\n Button names length: {} != button text length: {}", button_names.len(), button_text.len());
    
    let mut buttons = Vec::with_capacity(button_names.len());
    
    let offset = size.y;
    
    for i in 0..button_names.len() {
      let pos_y = position.y-offset*(i+1) as f32;
      let button = Button::new_button(button_names[i].to_string(), Vector2::new(position.x, pos_y), size, 
                   Vector2::new(size.x*0.5, size.y*0.15), button_text_size, 
                   Vector4::new(0.0, 0.0, 0.0, 1.0), button_colour-Vector4::new(0.2, 0.2, 0.2, 0.0), button_colour, 
                   false, true, button_text[i].to_string(), font.to_string());
      buttons.push(button);
    }
    
    let selected_button = Button::new_button(button_names[0].to_string(), position, size, 
                          Vector2::new(size.x*0.5, size.y*0.2), button_text_size, 
                          Vector4::new(0.0, 0.0, 0.0, 1.0), button_colour-Vector4::new(0.2, 0.2, 0.2, 0.0), button_colour, 
                          false, true, button_text[0].to_string(), font.to_string());
    
    let mut slider = None; 
    
    if size.y*button_names.len() as f32 > dropdown_size {
      slider = Some(Slider::_new("Scrollbar".to_string(), scroll_bar_primay_colour, scroll_bar_secondary_colour, Vector2::new(position.x+size.x*0.5+scroll_bar_width*0.5, position.y+size.y*0.5-dropdown_size*0.5), Vector2::new(scroll_bar_width, dropdown_size-size.y*0.5), Vector2::new(scroll_bar_width*0.75, 15.0), 0.0, scroll_speed, true));
    }
    
    DropdownBox {
      name: name,
      position: position,
      dropdown_size: dropdown_size,
      button_size: size,
      chosen_option: 0,
      selected_button: selected_button,
      buttons: buttons,
      is_dropped_down: false,
      scroll_bar: slider,
      dynamic: true,
    }
  }
  
  pub fn _name_matches(&self, name: &String) -> bool {
    &self.name == name
  }
  
  pub fn _get_selected_name(&self) -> String {
    self.selected_button.get_name()
  }
  
  pub fn _option_pressed_by_name(&self, name: &String) -> bool {
    let mut pressed = false;
    for button in &self.buttons {
      if button.name_matches(name) {
        pressed = button.is_pressed();
        break;
      }
    }
    pressed
  }
  
  pub fn _dropped_down(&self) -> bool {
    self.is_dropped_down
  }
  
  pub fn set_selected_button_by_index(&mut self, index: usize) {
    if self.dynamic {
      let new_pos = self.position;
      let text_pos = self.selected_button.get_text_location();
      self.selected_button = self.buttons[index].clone();
      self.selected_button.set_location(new_pos);
      self.selected_button.set_text_location(text_pos);
    }
  }
  
  pub fn _get_display_names(&self) -> Vec<String> {
    let mut display_names = Vec::with_capacity(self.buttons.len());
    for button in &self.buttons {
      display_names.push(button._get_text());
    }
    
    display_names
  }
  
  pub fn reset_scroll(&mut self) {
    let y_1 = self.buttons[0].get_location().y+self.button_size.y;
    let y_2 = self.position.y;
      
    if y_1 < y_2+1.0 {
      let length = self.buttons.len();
      for i in 0..length {
        let pos = self.buttons[i].get_location();
        let size = self.button_size.y;
        let y_3 = y_2-size-(i)as f32*size;
        self.buttons[i].set_location(Vector2::new(pos.x, y_3));
      }
    }
  }
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, scroll_delta: f32) {
    if self.selected_button.is_pressed() {
      self.is_dropped_down = true;
    }
    
    let selected_button_is_touching_mouse = self.selected_button.is_touching(mouse_pos);
    let scroll_bar_is_touching_mouse = {
      let mut touching = false;
      if let Some(ref scroll_bar) = self.scroll_bar {
        touching = scroll_bar.is_touching(mouse_pos);
      }
      touching
    };
    
    let any_button_touching_mouse = {
      let mut touching = false;
      for button in &self.buttons {
        if button.is_touching(mouse_pos) {
          touching = true;
          break;
        }
      }
      touching
    };
    
    if self.is_dropped_down {
      let mut close_drodown_box = true;
      
      if !left_mouse {
        close_drodown_box = false;
      } else {
        if selected_button_is_touching_mouse {
          close_drodown_box = false;
        } else if scroll_bar_is_touching_mouse {
          close_drodown_box = false
        } else {
          if any_button_touching_mouse {
            close_drodown_box = false;
          }
        }
      }
      
      if close_drodown_box {
        self.is_dropped_down = false;
      }
    
    
      if selected_button_is_touching_mouse || any_button_touching_mouse {
        let length = self.buttons.len();
       
        let size = self.button_size.y;
        
        let mut value = 0.0;
        if let Some(ref mut scroll_bar) = self.scroll_bar {
          scroll_bar.scroll(delta_time, scroll_delta);
          value = scroll_bar.get_value();
        }
        let mag = size+(length-1)as f32*size;
        
        let offset = value*mag;
        for button in &mut self.buttons {
          button.change_location(Vector2::new(0.0, offset));
        }
        
        let y_1 = self.buttons[self.buttons.len()-1].get_location().y-self.button_size.y;
        let y_2 = self.position.y-self.dropdown_size;
        
        if y_1 > y_2+1.0 {
          for i in 0..length {
            let pos = self.buttons[i].get_location();
            let y_3 = y_2+size+(length-1-i)as f32*size;
            self.buttons[i].set_location(Vector2::new(pos.x, y_3));
          }
        }
        
       
        self.reset_scroll();
      }
      
      if let Some(ref scroll_bar)  = self.scroll_bar {
        let scroll_value = scroll_bar.get_value();
        let dist = (((self.buttons.len()-1) as f32*self.button_size.y)-self.dropdown_size*0.5-self.button_size.y*0.5) * (1.0-scroll_value);
        let y_2 = self.position.y;
          
        let length = self.buttons.len();
        
        for i in 0..length {
          let pos = self.buttons[i].get_location();
          let size = self.button_size.y;
          let y_3 = y_2-size-(i)as f32*size;
          self.buttons[i].set_location(Vector2::new(pos.x, y_3+dist));
        }
      }
    }
    
    let mut should_reset_scroll = false;
    for i in 0..self.buttons.len() {
      let mut left_mouse_clicked = left_mouse;
      if !self.is_dropped_down {
        left_mouse_clicked = false;
      } else {
        let pos = self.buttons[i].get_location();
        let size = self.buttons[i].get_size();
        if pos.y > self.position.y-self.dropdown_size && pos.y-size.y*0.5 < self.position.y-size.y {
          if self.buttons[i].is_pressed() {
            self.set_selected_button_by_index(i);
            self.is_dropped_down = false;
            should_reset_scroll = true;
          }
        }
      }
      
      self.buttons[i].update(delta_time, mouse_pos, left_mouse_clicked);
    }
          
    if should_reset_scroll {
      self.reset_scroll();
    }
    
    self.selected_button.update(delta_time, mouse_pos, left_mouse);
    
    if let Some(ref mut scroll_bar) = self.scroll_bar {
      scroll_bar.update(delta_time, mouse_pos, left_mouse, scroll_delta);
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    if self.is_dropped_down {
      let pos = Vector2::new(self.position.x - self.button_size.x*0.5, self.position.y+self.button_size.y*0.5);
      let size = Vector2::new(self.button_size.x, self.dropdown_size);
      draw_calls.push(DrawCall::set_render_scissor(Vector4::new(pos.x, pos.y, size.x, size.y)));
      
      draw_calls.push(DrawCall::draw_coloured(Vector2::new(self.position.x, self.position.y-self.dropdown_size*0.5), Vector2::new(self.button_size.x+50.0, self.dropdown_size+50.0), Vector4::new(0.0, 0.0, 0.0, 1.0), 90.0));
      
      for button in &self.buttons {
        button.draw(draw_calls);
      }
      draw_calls.push(DrawCall::reset_render_scissor());
      if let Some(ref scroll_bar) = self.scroll_bar {
        scroll_bar.draw(draw_calls);
      } 
    }
    
    self.selected_button.draw(draw_calls);
  }
}
