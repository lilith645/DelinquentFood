use maat_graphics::DrawCall;
use maat_graphics::math;

use cgmath::Vector2;
use cgmath::Vector4;

use crate::modules::system_interface::TextField;
use crate::modules::system_interface::asserts;

#[derive(Clone, PartialEq)]
pub struct TexturedButton {
  pub pressed: String,
  pub unpressed: String,
  pub text_field: Option<TextField>,
}

impl TexturedButton {
  pub fn new(pressed: String, unpressed: String) -> TexturedButton {
    TexturedButton {
      pressed: pressed,
      unpressed: unpressed,
      text_field: None,
    }
  }
  
  pub fn _new_empty() -> TexturedButton {
    TexturedButton {
      pressed: "".to_string(),
      unpressed: "".to_string(),
      text_field: None,
    }
  }
  
  pub fn _with_text_field(_position: Vector2<f32>, _size: Vector2<f32>, _colour: Vector4<f32>) {
    
  }
}

#[derive(Clone, PartialEq)]
pub struct ColouredButton {
  pub pressed: Vector4<f32>,
  pub unpressed: Vector4<f32>,
  pub text_field: Option<TextField>,
}

impl ColouredButton {
  pub fn new(name: String, position: Vector2<f32>, size: Vector2<f32>, text_colour: Vector4<f32>, pressed: Vector4<f32>, unpressed: Vector4<f32>, centered: bool, text: String, font: String) -> ColouredButton {
    ColouredButton {
      pressed: pressed,
      unpressed: unpressed,
      text_field: Some(TextField::new_text_field(name, position, size, text_colour, centered, text, font)),
    }
  }
  
  pub fn _new_no_text(pressed: Vector4<f32>, unpressed: Vector4<f32>) -> ColouredButton {
    ColouredButton {
      pressed: pressed,
      unpressed: unpressed,
      text_field: None,
    }
  }
  
  pub fn _new_empty() -> ColouredButton {
    ColouredButton {
      pressed: Vector4::new(0.0, 0.0, 0.0, 0.0),
      unpressed: Vector4::new(0.0, 0.0, 0.0, 0.0),
      text_field: None,
    }
  }
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
  Textured(TexturedButton),
  Coloured(ColouredButton),
}

impl ButtonType {
  pub fn _get_colours(&self) -> Option<(Vector4<f32>, Vector4<f32>)>{
    let colours;
    match self {
      ButtonType::Coloured(details) => {
        let pressed = details.pressed;
        let unpressed = details.unpressed;
        colours = Some((pressed, unpressed));
      },
      ButtonType::Textured(_details) => {
        colours = None;
      }
    }
    colours
  }
  
  pub fn _get_text(&self) -> String {
    let mut text = "".to_string();
    
    match self {
      ButtonType::Coloured(details) => {
        if let Some(ref text_field) = details.text_field {
          text = text_field.get_text();
        }
      },
      ButtonType::Textured(details) => {
        if let Some(ref text_field) = details.text_field {
          text = text_field.get_text();
        }
      }
    }
    text
  }
  
  pub fn _add_textfield(&mut self, textfield: TextField) {
    match self {
      ButtonType::Coloured(details) => {
        details.text_field = Some(textfield);
      },
      ButtonType::Textured(details) => {
        details.text_field = Some(textfield);
      }
    }
  }
}

#[derive(Clone, PartialEq)]
pub struct Button {
  name: String,
  position: Vector2<f32>,
  size: Vector2<f32>,
  button_type: ButtonType,
  is_toggle: bool,
  toggled: bool,
  has_background: bool,
  background_colour: Vector4<f32>,
  background_size: Vector2<f32>,
  pressed_last_frame: bool, // Left mouse button pressed last frame
  released_this_frame: bool, // Left mouse button released this frame
  should_update: bool,
}

impl Button {
  pub fn new_button(name: String, position: Vector2<f32>, size: Vector2<f32>, 
                    relative_position: Vector2<f32>, text_size: Vector2<f32>, 
                    text_colour: Vector4<f32>, pressed: Vector4<f32>, unpressed: Vector4<f32>, 
                    is_toggle: bool, center_text: bool, text: String, font: String) -> Button {
    asserts::vec2_greater_than(&size, &0.0, "Button size is required to be larger than 0");
    asserts::vec2_greater_than(&text_size, &0.0, "Text size of button is required to be larger than 0");
    asserts::check_colour_range_vec4(&text_colour);
    asserts::check_colour_range_vec4(&pressed);
    asserts::check_colour_range_vec4(&unpressed);

    let pos = (position-size*0.5) + relative_position;
    let name = name;
    Button {
      name: name.to_string(),
      position: position,
      size: size,
      button_type: ButtonType::Coloured(ColouredButton::new(name.to_string(), pos, text_size, text_colour, 
                                                            pressed, unpressed, center_text, text, font)),
      is_toggle: is_toggle,
      toggled: false,
      has_background: false,
      background_colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      background_size: Vector2::new(0.0, 0.0),
      pressed_last_frame: false,
      released_this_frame: false,
      should_update: true,
    }
  }
  
 pub fn _new_button_no_text(name: String, position: Vector2<f32>, size: Vector2<f32>, 
                           pressed: Vector4<f32>, unpressed: Vector4<f32>, is_toggle: bool) -> Button {
    asserts::vec2_greater_than(&size, &0.0, "Button size is required to be larger than 0");
    asserts::check_colour_range_vec4(&pressed);
    asserts::check_colour_range_vec4(&unpressed);
    let name = name;
    Button {
      name: name.to_string(),
      position: position,
      size: size,
      button_type: ButtonType::Coloured(ColouredButton::_new_no_text(pressed, unpressed)),
      is_toggle: is_toggle,
      toggled: false,
      has_background: false,
      background_colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      background_size: Vector2::new(0.0, 0.0),
      pressed_last_frame: false,
      released_this_frame: false,
      should_update: true,
    }
  }
  
 pub fn _new_button_no_text_with_background(name: String, position: Vector2<f32>, size: Vector2<f32>, 
                           pressed: Vector4<f32>, unpressed: Vector4<f32>, is_toggle: bool, background_colour: Vector4<f32>, background_size: Vector2<f32>) -> Button {
    asserts::vec2_greater_than(&size, &0.0, "Button size is required to be larger than 0");
    asserts::check_colour_range_vec4(&pressed);
    asserts::check_colour_range_vec4(&unpressed);
    let name = name;
    Button {
      name: name.to_string(),
      position: position,
      size: size,
      button_type: ButtonType::Coloured(ColouredButton::_new_no_text(pressed, unpressed)),
      is_toggle: is_toggle,
      toggled: false,
      has_background: true,
      background_colour: background_colour,
      background_size: background_size,
      pressed_last_frame: false,
      released_this_frame: false,
      should_update: true,
    }
  }
  
  pub fn new_textured_button(name: String, position: Vector2<f32>, size: Vector2<f32>, pressed: String, unpressed: String, is_toggle: bool) -> Button {
    asserts::vec2_greater_than(&size, &0.0, "Button size is required to be larger than 0");
    Button {
      name: name,
      position: position,
      size: size,
      button_type: ButtonType::Textured(TexturedButton::new(pressed, unpressed)),
      is_toggle: is_toggle,
      toggled: false,
      has_background: false,
      background_colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      background_size: Vector2::new(0.0, 0.0),
      pressed_last_frame: false,
      released_this_frame: false,
      should_update: true,
    }
  }
  
  pub fn _new_textured_button_with_background(name: String, position: Vector2<f32>, size: Vector2<f32>, pressed: String, unpressed: String, is_toggle: bool, background_colour: Vector4<f32>, background_size: Vector2<f32>) -> Button {
    asserts::vec2_greater_than(&size, &0.0, "Button size is required to be larger than 0");
    Button {
      name: name,
      position: position,
      size: size,
      button_type: ButtonType::Textured(TexturedButton::new(pressed, unpressed)),
      is_toggle: is_toggle,
      toggled: false,
      has_background: true,
      background_colour: background_colour,
      background_size: background_size,
      pressed_last_frame: false,
      released_this_frame: false,
      should_update: true,
    }
  }
  
  pub fn _new_empty() -> Button {
    Button {
      name: "".to_string(),
      position: Vector2::new(0.0, 0.0),
      size: Vector2::new(0.0, 0.0),
      button_type: ButtonType::Coloured(ColouredButton::_new_empty()),
      is_toggle: false,
      toggled: false,
      has_background: false,
      background_colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      background_size: Vector2::new(0.0, 0.0),
      pressed_last_frame: false,
      released_this_frame: false,
      should_update: true,
    }
  }
  
  pub fn _get_all(&self) -> (String, Vector2<f32>, Vector2<f32>, ButtonType, bool, bool, bool, Vector4<f32>, Vector2<f32>, bool, bool, bool) {
    let name = self.name.to_string();
    let pos = self.position;
    let size = self.size;
    let button_type = self.button_type.clone();
    let is_toggle = self.is_toggle;
    let toggled = self.toggled;
    let has_background = self.has_background;
    let background_colour = self.background_colour;
    let background_size = self.background_size;
    let pressed_last_frame = false;
    let released_this_frame = false;
    let should_update = true;
    
    (name, pos, size, button_type, is_toggle, toggled, has_background, background_colour, background_size, pressed_last_frame, released_this_frame, should_update)
  }
  
  pub fn _add_textfield(&mut self, textfield: TextField) {
    self.button_type._add_textfield(textfield);
  }
  
  /*
  pub fn _new_button_coloured(&mut self, name: String, position: Vector2<f32>, size: Vector2<f32>, _pressed_colour: Vector4<f32>, _unpressed_colour: Vector4<f32>, is_toggle: bool, _text: String) -> Button {
    Button {
      name: name,
      text_fields: Vec::new(),
      position: position,
      size: size,
      unpressed_texture: "".to_string(),
      pressed_texture: "".to_string(),
      is_toggle: is_toggle,
      toggled: false,
      has_background: false,
      background_colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      background_size: Vector2::new(0.0, 0.0),
    }
  }*/
  
  pub fn _with_text_field(self, _name: String, _relative_position: Vector2<f32>, _size: Vector2<f32>, _colour: Vector4<f32>, _text: String, _font: String) -> Button {
    panic!("Unimplmented Function with_text_field(), Button.rs ~line 118");
    //  let pos = (self.position-self.size*0.5) + relative_position;
    // self.text_fields.push(TextField::new_text_field(name, pos, size, colour, text, font));
    //self
  }
  
  pub fn get_location(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn get_size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn _get_text(&self) -> String {
    self.button_type._get_text()
  }
  
  pub fn _set_size(&mut self, new_size: Vector2<f32>) {
    asserts::vec2_greater_than(&new_size, &0.0, "Button size needs to be greater than 0,0");
    self.size = new_size;
  }
  
  pub fn get_text_location(&self) -> Vector2<f32> {
    let mut text_pos = Vector2::new(0.0, 0.0);
    match self.button_type {
      ButtonType::Coloured(ref details) => {
        if let Some(ref text_field) = details.text_field {
          text_pos = text_field.get_location();
        }
      },
      ButtonType::Textured(ref details) => {
        if let Some(ref text_field) = details.text_field {
          text_pos = text_field.get_location();
        }
      }
    }
    
    text_pos
  }
  
  pub fn set_location(&mut self, location: Vector2<f32>) {
    let text_location = self.get_text_location();
    let button_location = self.position;
    let new_button_location = location;
    
    let diff_text_button = text_location-button_location;
    
    self.position = new_button_location;
    self.set_text_location(new_button_location+diff_text_button);
  }
  
  pub fn change_location(&mut self, location_offset: Vector2<f32>) {
     self.position += location_offset;
     let text_pos = self.get_text_location();
     let new_pos = text_pos + location_offset;
     self.set_text_location(new_pos);
  }
  
  pub fn set_text_location(&mut self, location: Vector2<f32>) {
    let mut coloured_button = None;
    let mut textured_button = None;
    
    match self.button_type {
      ButtonType::Coloured(ref mut details) => {
        if details.text_field.is_some() {
          let pressed = details.pressed.clone();
          let unpressed = details.unpressed.clone();
          let mut text_field = details.text_field.clone().unwrap();
          text_field.set_location(location);
          coloured_button = Some(ButtonType::Coloured(ColouredButton {pressed: pressed, unpressed: unpressed, text_field: Some(text_field) }));
        }
      },
      ButtonType::Textured(ref mut details) => {
        if details.text_field.is_some() {
          let pressed = details.pressed.clone();
          let unpressed = details.unpressed.clone();
          let mut text_field = details.text_field.clone().unwrap();
          text_field.set_location(location);
          textured_button = Some(ButtonType::Textured(TexturedButton {pressed: pressed, unpressed: unpressed, text_field: Some(text_field) }));
        }
      }
    }
    
    if coloured_button.is_some() {
      self.button_type = coloured_button.unwrap();
    } else if textured_button.is_some() {
      self.button_type = textured_button.unwrap();
    }
  }
  
  pub fn _toggled_on(mut self) -> Button {
    self.toggled = true;
    self
  }
  
  pub fn _toggle_background(mut self, size: Vector2<f32>, colour: Vector4<f32>) -> Button {
    asserts::check_colour_range_vec4(&colour);
    self.background_colour = colour;
    self.background_size = size;
    self.has_background = true;
    self
  }
  
  pub fn toggle_on(&mut self) {
    self.toggled = true;
  }
  
  pub fn toggle_off(&mut self) {
    self.toggled = false;
  }
  
  pub fn is_toggled(&self) -> bool {
    let mut selected = false;
    if self.is_toggle {
      selected = self.toggled;
    }
    
    selected
  }
  
  pub fn _stop_updating(&mut self) {
    self.should_update = false;
  }
  
  pub fn is_pressed(&self) -> bool {
    let pressed;
    
    // If is regular button
    if !self.is_toggle { 
      pressed = self.released_this_frame;
    } else {
      pressed = self.toggled;
    }
    
    pressed
  }
  
  pub fn is_held(&self) -> bool {
    let held;
    if self.toggled {
      held = true;
    } else {
      held = false;
    }
    held
  }
  
  pub fn is_touching(&self, at_location: Vector2<f32>) -> bool {
    let center_x = self.position.x;
    let center_y = self.position.y;
    let width = self.size.x;
    let height = self.size.y;
    
    math::box_collision(Vector4::new(center_x, center_y, width, height), Vector4::new(at_location.x, at_location.y, 1.0, 1.0))
  }
  
  pub fn name_matches(&self, name: &String) -> bool {
    &self.name == name
  }
  
  pub fn _set_name(&mut self, name: &String) {
    self.name = name.clone();
  }
  
  pub fn _get_button_colours(&self) -> Option<(Vector4<f32>, Vector4<f32>)> {
    self.button_type._get_colours()
  }
  
  pub fn _set_colour(&mut self, pressed: Vector4<f32>, unpressed: Vector4<f32>) {
    asserts::check_colour_range_vec4(&pressed);
    asserts::check_colour_range_vec4(&unpressed);
    
    match &mut self.button_type {
      ButtonType::Coloured(details) => {
        details.pressed = pressed;
        details.unpressed = unpressed;
      }, 
      _ => {},
    }
  }
  
  pub fn get_name(&self) -> String {
    self.name.to_string()
  }
  
  pub fn update(&mut self, _delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool) {
    if !self.should_update {
      return;
    }
    
    if self.pressed_last_frame && !left_mouse {
      if self.is_touching(mouse_pos) {
        self.released_this_frame = true;
      }
    } else {
      self.released_this_frame = false;
    }
    
    // if regular button
    if !self.is_toggle {
      if left_mouse {
        if self.is_touching(mouse_pos) {
          self.toggle_on();
        } else {
          self.toggle_off();
        }
      } else {
        self.toggle_off();
      }
    } else {
      if self.released_this_frame && self.is_touching(mouse_pos) {
        if self.is_toggled() {
          self.toggle_off();
        } else {
          self.toggle_on();
        }
      }
    }
    
    self.pressed_last_frame = left_mouse;
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    if self.has_background && self.is_toggle && self.toggled {
      let pos = self.position;
      let size = self.background_size;
      let colour = self.background_colour;
      draw_calls.push(DrawCall::draw_coloured(pos, size, colour, 90.0));
    }
    
    let pos = self.position;
    let size = self.size;
    let texture;
    
    let is_pressed = self.toggled;
    
    match self.button_type {
      ButtonType::Textured(ref data) => {
        if is_pressed {
          let temp = &data.pressed;
          texture = temp.to_string();
        } else {
          let temp = &data.unpressed;
          texture = temp.to_string();
        }
        draw_calls.push(DrawCall::draw_textured(pos, size, 90.0, texture));
      },
      ButtonType::Coloured(ref data) => {
        let colour;
        if is_pressed {
          colour = data.pressed;
        } else {
          colour = data.unpressed;
        }
        draw_calls.push(DrawCall::draw_coloured(pos, size-Vector2::new(1.0, 1.0), colour, 90.0));
        if let Some(text) = &data.text_field {
          text.draw(draw_calls);
        }
      }
    }
  }
}
