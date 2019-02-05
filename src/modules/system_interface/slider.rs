use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::modules::system_interface::asserts;
use crate::modules::system_interface::Button;

use cgmath::Vector2;
use cgmath::Vector4;

#[derive(Clone)]
pub struct Slider {
  name: String,
  primary_colour: Vector4<f32>,
  position: Vector2<f32>,
  bar_size: Vector2<f32>,
  ball: Button,
  value: f32, // 0 to 1
  scroll_speed: f32,
  is_vertical: bool,
}

impl Slider {
  pub fn _new(name: String, primary_colour: Vector4<f32>, secondary_colour: Vector4<f32>, position: Vector2<f32>,
             bar_size: Vector2<f32>, ball_size: Vector2<f32>, value: f32, scroll_speed: f32, vertical: bool) -> Slider {
    debug_assert!(!(value > 1.0 || value < 0.0), "Error slider value can only be between 0 and 1");
    asserts::check_colour_range_vec4(&primary_colour);
    asserts::check_colour_range_vec4(&secondary_colour);
    asserts::vec2_greater_than(&bar_size, &0.0, "Bar size must be greater than 0");
    asserts::vec2_greater_than(&ball_size, &0.0, "Bar size must be greater than 0");
    let mut value = value;
    let name = name;
    let button;
    if vertical {
      value = value;
      button = Button::_new_button_no_text(name.to_string(), Vector2::new(position.x, 0.0), ball_size, secondary_colour, secondary_colour, false);
    } else {
      button = Button::_new_button_no_text(name.to_string(), Vector2::new(0.0, position.y), ball_size, secondary_colour, secondary_colour, false);
    }
    
    let mut slider = Slider {
      name: name.to_string(),
      primary_colour: primary_colour,
      position: position,
      bar_size: bar_size,
      ball: button,
      value: value, // 0 to 1
      scroll_speed: scroll_speed,
      is_vertical: vertical,
    };
    
    slider.set_value(value);
    slider
    
  }
  
  pub fn is_touching(&self, at_location: Vector2<f32>) -> bool {
    let center_x = self.position.x;
    let center_y = self.position.y;
    let width = self.bar_size.x;
    let height = self.bar_size.y;
    
    math::box_collision(Vector4::new(center_x, center_y, width, height), Vector4::new(at_location.x, at_location.y, 1.0, 1.0))
  }
  
  pub fn get_value(&self) -> f32 {
    self.value
  }
  
  pub fn _get_bar_location(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn _get_ball_location(&self) -> Vector2<f32> {
    println!("Value: {}", self.value);
    self.ball.get_location()
  }
  
  pub fn _get_bar_size(&self) -> Vector2<f32> {
    self.bar_size
  }
  
  pub fn _get_ball_size(&self) -> Vector2<f32> {
    self.ball.get_size()
  }
  
  pub fn set_value(&mut self, value: f32) {
    debug_assert!(!(value > 1.0 || value < 0.0), "Error slider value can only be between 0 and 1. actual value: {}", value);
    debug_assert!(!value.is_nan(), "Value must be a number. actual value: {}", value);
    let x = self.ball.get_location().x;
    let y = self.ball.get_location().y;
    let bar_size = self.bar_size;
    let ball_size = self.ball.get_size();
    let pos = self.position;
     
    if self.is_vertical {
       let corrected_bar_height = bar_size.y-ball_size.y;
       let new_y = pos.y-corrected_bar_height*0.5 + (corrected_bar_height*value);
       self.ball.set_location(Vector2::new(x, new_y));
    } else {
       let corrected_bar_width = bar_size.x-ball_size.x;
       let new_x = pos.x+corrected_bar_width*0.5 - (corrected_bar_width*value);
       self.ball.set_location(Vector2::new(new_x, y));
    }
    
    self.value = value;
  }
  
  pub fn scroll(&mut self, _delta_time: f32, scroll_delta: f32) {
    let mut new_value = self.value;
    let offset = scroll_delta*self.scroll_speed;
    new_value += offset;
    if new_value > 1.0 {
      new_value = 1.0;
    }
    if new_value < 0.0 {
      new_value = 0.0;
    }
    self.set_value(new_value);
  }
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, scroll_delta: f32) {
    if self.ball.is_held() || left_mouse && self.is_touching(mouse_pos) {
      if !self.is_vertical {
        let mut loc = Vector2::new(mouse_pos.x, self.ball.get_location().y); 
        
        let bar_pos = self.position;
        let bar_size = self.bar_size;
        let ball_size = self.ball.get_size();
        
        let corrected_bar_size = Vector2::new(bar_size.x-ball_size.x, bar_size.y);
        
        if loc.x < bar_pos.x-corrected_bar_size.x*0.5 {
          loc.x = bar_pos.x-corrected_bar_size.x*0.5;
        }
        if loc.x > bar_pos.x+corrected_bar_size.x*0.5 {
          loc.x = bar_pos.x+corrected_bar_size.x*0.5;
        }
        
        let dist = bar_pos.x+corrected_bar_size.x*0.5-loc.x;
        let value = dist/corrected_bar_size.x;
        
        self.set_value(value);
      } else {
        let mut loc = Vector2::new(self.ball.get_location().x, mouse_pos.y); 
        let lower_pos = self.position-self.bar_size*0.5;
        let higher_pos = self.position+self.bar_size*0.5;
        if loc.y < lower_pos.y+self.ball.get_size().y*0.5 {
          loc.y = lower_pos.y+self.ball.get_size().y*0.5;
        }
        if loc.y > higher_pos.y-self.ball.get_size().y*1.5 {
          loc.y = higher_pos.y-self.ball.get_size().y*1.5;
        }
        
        let dist = loc.y - lower_pos.y;
        let max_dist = self.bar_size.y-self.ball.get_size().y;
        let value = dist/max_dist;
        
        self.set_value(value);
      }
    } else if self.is_touching(mouse_pos) {
      self.scroll(delta_time, scroll_delta);
    }
    
    self.ball.update(delta_time, mouse_pos, left_mouse);
  }
  
  pub fn _name_matches(&self, name: &String) -> bool {
    &self.name == name
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let bar_pos = self.position;
    let bar_size = self.bar_size;
    let p_colour = self.primary_colour;
    
    //bar
    draw_calls.push(DrawCall::draw_coloured(bar_pos, bar_size, p_colour, 90.0));
    //ball
    self.ball.draw(draw_calls);
  }
}
