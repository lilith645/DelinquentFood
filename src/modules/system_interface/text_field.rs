use maat_graphics::math;
use maat_graphics::DrawCall;

use cgmath::Vector2;
use cgmath::Vector4;

#[derive(Clone, PartialEq)]
pub struct TextField {
  name: String,
  text: String,
  font: String,
  wrap: bool,
  centered: bool,
  position: Vector2<f32>,
  size: Vector2<f32>,
  colour: Vector4<f32>,
  editable: bool,
  editing: bool,
}


impl TextField {
  pub fn new_text_field(name: String, position: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>, centered: bool, text: String, font: String) -> TextField {
    TextField {
      name: name,
      text: text,
      font: font,
      wrap: false,
      centered: centered,
      position: position,
      size: size,
      colour: colour,
      editable: false,
      editing: false,
    }
  }
  
  pub fn _new_text_field_editable(name: String, position: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>, centered: bool, text: String, font: String) -> TextField {
    
    println!("editable made");
    TextField {
      editable: true,
      .. TextField::new_text_field(name, position, size, colour, centered, text, font)
    }
  }
  
  pub fn _new_empty() -> TextField {
    TextField {
      name: "".to_string(),
      text: "".to_string(),
      font: "".to_string(),
      wrap: false,
      centered: false,
      position: Vector2::new(0.0, 0.0),
      size: Vector2::new(0.0, 0.0),
      colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      editable: false,
      editing: false
    }
  }
  
  pub fn is_touching(&self, at_location: Vector2<f32>) -> bool {
    let size = self.size;
    let width = size.x*0.06*self.text.len() as f32;
    let height = size.y*0.15;
    let center_x = self.position.x;
    let center_y = self.position.y+height*0.5;

    math::box_collision(Vector4::new(center_x, center_y, width, height), Vector4::new(at_location.x, at_location.y, 1.0, 1.0))
  }
  
  pub fn _get_all(&self) -> (String, String, String, bool, bool, Vector2<f32>, Vector2<f32>, Vector4<f32>) {
    let name = self.name.to_string();
    let text = self.text.to_string();
    let font = self.font.to_string();
    let wrap = self.wrap;
    let centered = self.centered;
    let pos = self.position;
    let size = self.size;
    let colour = self.colour;
    
    (name, text, font, wrap, centered, pos, size, colour)
  }
  
  pub fn _wrap_text(mut self, should_wrap: bool) -> TextField {
    self.wrap = should_wrap;
    self
  }
  
  pub fn name_matches(&self, name: &String) -> bool {
    &self.name == name
  }
  
  pub fn _set_colour(&mut self, new_colour: Vector4<f32>) {
    self.colour = new_colour;
  }
  
  pub fn _set_name(&mut self, new_name: &String) {
    self.name = new_name.to_string();
  }
  
  pub fn update_text(&mut self, new_text: String) {
    self.text = new_text;
  }
  
  pub fn _set_text_size(&mut self, new_size: Vector2<f32>) { 
    self.size = new_size;
  }
  
  pub fn get_location(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn set_location(&mut self, new_location: Vector2<f32>) {
    self.position = new_location;
  }
  
  pub fn get_text(&self) -> String {
    self.text.clone()
  }
  
  pub fn _get_colour(&self) -> Vector4<f32> {
    self.colour
  }
  
  pub fn _get_size(&self) -> f32 {
    self.size.x
  }
  
  pub fn _get_name(&self) -> String {
    self.name.to_string()
  }
  
  pub fn update(&mut self, _delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, keys_pressed_this_frame: &Vec<String>) {
    if self.editable {
      if self.editing {
        for i in 0..keys_pressed_this_frame.len() {
          if keys_pressed_this_frame[i] == "Backspace" {
            self.text.pop();
          } else if keys_pressed_this_frame[i] == "Enter" {
            self.text = self.text.to_owned() + &"\n";
          } else {
            self.text = self.text.to_owned() + &keys_pressed_this_frame[i].to_string();
          }
        }
      }
      
      if left_mouse {
        if self.is_touching(mouse_pos) {
          self.editing = true;
        } else {
          self.editing = false;
        }
      }
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let pos = self.position;
    let _size = self.size;
    let text = self.text.clone();
    let font = self.font.clone();
    let colour = self.colour;
    let _wrap = self.wrap;
    let size = self.size;
    let _width = size.x*0.06*self.text.len() as f32;
    let height = size.y*0.15;
    let _center_x = self.position.x;
    let _center_y = self.position.y+height*0.5;
    
   // draw_calls.push(DrawCall::draw_coloured(Vector2::new(center_x,center_y), Vector2::new(width, height), Vector4::new(1.0, 1.0, 1.0, 1.0), 90.0));
    
    if self.centered {
      draw_calls.push(DrawCall::draw_text_basic_centered(pos, size, colour, text , font));
    } else {
      draw_calls.push(DrawCall::draw_text_basic(pos, size, colour, text , font));
    }
  }
}
