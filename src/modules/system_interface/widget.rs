use maat_graphics::DrawCall;
use maat_graphics::math;

use crate::modules::system_interface::Selection;
use crate::modules::system_interface::Button;
use crate::modules::system_interface::TextField;
use crate::modules::system_interface::Slider;
use crate::modules::system_interface::DropdownBox;

use cgmath::Vector2;
use cgmath::Vector4;

#[derive(Clone)]
pub struct Widget {
  _name: String,
  position: Vector2<f32>,
  size: Vector2<f32>,
  colour: Vector4<f32>,
  hidden: bool,
  text_fields: Vec<TextField>,
  buttons: Vec<Button>,
  selections: Vec<Selection>,
  sliders: Vec<Slider>,
  dropdown_boxs: Vec<DropdownBox>,
}

impl Widget {
  pub fn new(name: String, position: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>) -> Widget {
    Widget {
      _name: name,
      position: position,
      size: size,
      colour: colour,
      hidden: false,
      text_fields: Vec::new(),
      buttons: Vec::new(),
      selections: Vec::new(),
      sliders: Vec::new(),
      dropdown_boxs: Vec::new(),
    }
  }
  
  pub fn _new_empty() -> Widget {
    Widget {
      _name: "".to_string(),
      position: Vector2::new(0.0, 0.0),
      size: Vector2::new(0.0, 0.0),
      colour: Vector4::new(0.0, 0.0, 0.0, 0.0),
      hidden: false,
      text_fields: Vec::new(),
      buttons: Vec::new(),
      selections: Vec::new(),
      sliders: Vec::new(),
      dropdown_boxs: Vec::new(),
    }
  }
  
  pub fn _new_all(name: String, position: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>, textfields: Vec<TextField>, buttons: Vec<Button>, selections: Vec<Selection>, sliders: Vec<Slider>, dropdown_boxs: Vec<DropdownBox>) -> Widget {
    Widget {
      _name: name,
      position: position,
      size: size,
      colour: colour,
      hidden: false,
      text_fields: textfields,
      buttons: buttons,
      selections: selections,
      sliders: sliders,
      dropdown_boxs: dropdown_boxs,
    }
  }
  
  pub fn _with_upwards_selection(mut self, relative_position: Vector2<f32>, option_size: Vector2<f32>, relative_text_position: Vector2<f32>, text_size: Vector2<f32>, text_colour: Vector4<f32>, spacing: f32, option_names: Vec<String>, option_colours: Vec<Vector4<f32>>, selected_background: Vector4<f32>, center_text: bool, text: Vec<String>, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.selections.push(Selection::_new_upwards(pos, option_size, relative_text_position, text_size, 
                                                text_colour, spacing, option_names, option_colours, 
                                                selected_background, center_text, text, font));
    self
  }
  
  pub fn _with_upwards_selection_textured(mut self, relative_position: Vector2<f32>, option_size: Vector2<f32>, spacing: f32, option_names: Vec<String>, option_textures: Vec<String>) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.selections.push(Selection::_new_upwards_textured(pos, option_size, spacing, option_names, option_textures, Vector4::new(0.0, 0.0, 1.0, 1.0)));
    self
  }
  
  pub fn with_button(mut self, relative_position: Vector2<f32>, button_size: Vector2<f32>, 
                     text_relative_position: Vector2<f32>, text_size: Vector2<f32>, 
                     name: String, text_colour: Vector4<f32>, pressed_button: Vector4<f32>, 
                     unpressed_button: Vector4<f32>, center_text: bool, text: String, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.buttons.push(Button::new_button(name, pos, button_size, text_relative_position, text_size, text_colour, pressed_button, unpressed_button, false, center_text, text, font));
    self
  }
  
  pub fn _with_textured_button(mut self, relative_position: Vector2<f32>, button_size: Vector2<f32>, button_name: String, pressed_button_texture: String, unpressed_button_texture: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.buttons.push(Button::new_textured_button(button_name, pos, button_size, pressed_button_texture, unpressed_button_texture, false));
    self
  }
  
  pub fn with_toggle_button_textured(mut self, relative_position: Vector2<f32>, button_size: Vector2<f32>, button_name: String, pressed_button_texture: String, unpressed_button_texture: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.buttons.push(Button::new_textured_button(button_name, pos, button_size, pressed_button_texture, unpressed_button_texture, true));
    self
  }
  
  pub fn _with_slider(mut self, relative_position: Vector2<f32>, bar_size: Vector2<f32>, ball_size: Vector2<f32>, slider_name: String, primary_colour: Vector4<f32>, secondary_colour: Vector4<f32>, value: f32, scroll_speed: f32, vertical: bool) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.sliders.push(Slider::_new(slider_name, primary_colour, secondary_colour, pos, bar_size, ball_size, value, scroll_speed, vertical));
    self
  }
  
  pub fn _with_dropdown_box(mut self, name: String, relative_position: Vector2<f32>, size: Vector2<f32>, 
                           dropdown_size: f32, scroll_speed: f32, button_text_size: Vector2<f32>, button_colour: Vector4<f32>,
                           scroll_bar_width: f32, scroll_bar_primay_colour: Vector4<f32>, scroll_bar_secondary_colour: Vector4<f32>, 
                           selected_name: String, selected_text: String, button_names: Vec<String>, button_text: Vec<String>, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.dropdown_boxs.push(DropdownBox::_new(name, pos, size, dropdown_size, scroll_speed, button_text_size, button_colour, scroll_bar_width, scroll_bar_primay_colour, scroll_bar_secondary_colour, selected_name, selected_text, button_names, button_text, font));
    self
  }
  
  pub fn _with_dropdown_box_dynamic(mut self, name: String, relative_position: Vector2<f32>, size: Vector2<f32>, 
                           dropdown_size: f32, scroll_speed: f32, button_text_size: Vector2<f32>, button_colour: Vector4<f32>,
                           scroll_bar_width: f32, scroll_bar_primay_colour: Vector4<f32>, scroll_bar_secondary_colour: Vector4<f32>, 
                           button_names: Vec<String>, button_text: Vec<String>, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.dropdown_boxs.push(DropdownBox::_new_dynamic(name, pos, size, dropdown_size, scroll_speed, button_text_size, button_colour, scroll_bar_width, scroll_bar_primay_colour, scroll_bar_secondary_colour, button_names, button_text, font));
    self
  }
  
  pub fn start_hidden(mut self) -> Widget {
    self.hidden = true;
    self
  }
  
  pub fn with_text_field(mut self, text_name: String, relative_position: Vector2<f32>, text_size: Vector2<f32>, text_colour: Vector4<f32>, text: String, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.text_fields.push(TextField::new_text_field(text_name, pos, text_size, text_colour, false, text, font));
    
    self
  }
  
  pub fn _with_text_field_editable(mut self, text_name: String, relative_position: Vector2<f32>, text_size: Vector2<f32>, text_colour: Vector4<f32>, text: String, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.text_fields.push(TextField::_new_text_field_editable(text_name, pos, text_size, text_colour, false, text, font));
    
    self
  }
  
  pub fn with_text_field_centered(mut self, text_name: String, relative_position: Vector2<f32>, text_size: Vector2<f32>, text_colour: Vector4<f32>, text: String, font: String) -> Widget {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.text_fields.push(TextField::new_text_field(text_name, pos, text_size, text_colour, true, text, font));
    
    self
  }
  
  pub fn is_touching(&self, at_location: Vector2<f32>) -> bool {
    let center_x = self.position.x;
    let center_y = self.position.y;
    let width = self.size.x;
    let height = self.size.y;
    
    math::box_collision(Vector4::new(center_x, center_y, width, height), Vector4::new(at_location.x, at_location.y, 1.0, 1.0))
  }
  
  pub fn _clear_widget(&mut self) {
    self.text_fields.clear();
    self.buttons.clear();
    self.selections.clear();
    self.sliders.clear();
    self.dropdown_boxs.clear();
  }
  
  pub fn _get_widget_details(&self) -> (String, Vector2<f32>, Vector2<f32>, Vector4<f32>) {
    let name = self._name.to_string();
    let position = self.position;
    let size = self.size;
    let colour = self.colour;
    
    (name, position, size, colour)
  }
  
  pub fn _get_widget_position(&self) -> Vector2<f32> {
    self.position
  }
  
  pub fn _get_widget_size(&self) -> Vector2<f32> {
    self.size
  }
  
  pub fn hide(&mut self) {
    self.hidden = true;
  }
  
  pub fn show(&mut self) {
    self.hidden = false;
  }
  
  pub fn _is_hidden(&self) -> bool {
    self.hidden
  }
  
  pub fn _get_widget_location(&self) -> Vector2<f32> {
    (self.position-self.size*0.5)
  }
  

  
  
  // 
  // DROPDOWN BOX
  //
  
  pub fn _get_selected_dropdown_box_option_i(&self, dropdown_box_set: usize) -> String {
    self.dropdown_boxs[dropdown_box_set]._get_selected_name()
  }
  
  pub fn _get_dropdown_box_by_name(&self, name: &String) -> Option<DropdownBox> {
    let mut dropdown_box = None;
    for i in 0..self.dropdown_boxs.len() {
      if self.dropdown_boxs[i]._name_matches(name) {
        dropdown_box = Some(self.dropdown_boxs[i].clone());
        break;
      }
    }
    dropdown_box
  }
  
  pub fn _set_dropdown_box_by_name_selected_option_by_index(&mut self, name: &String, index: usize) {
    for dropdown in &mut self.dropdown_boxs {
      if dropdown._name_matches(name) {
        dropdown.set_selected_button_by_index(index);
        break;
      }
    }
  }
  
  // 
  // TEXTFIELD
  //
  
  pub fn _is_touching_any_textfield(&self, at_location: Vector2<f32>) -> (bool, usize) {
    let mut is_touching = false;
    let mut i = 0;
    for text in &self.text_fields {
      is_touching = text.is_touching(at_location);
      if is_touching {
        break;
      }
      i += 1;
    }
    
    (is_touching, i)
  }
  
  pub fn _update_textfield_location(&mut self, name: &String, pos: Vector2<f32>) {
    let pos = pos;
    for text in &mut self.text_fields {
      if text.name_matches(name) {
        text.set_location(pos);
        break;
      }
    }
  }
  
  pub fn update_text_field(&mut self, name: &String, new_text: String) {
    for text in &mut self.text_fields {
      if text.name_matches(name) {
        text.update_text(new_text);
        break;
      }
    }
  }
  
  pub fn _update_text_field_by_index(&mut self, index: usize, new_text: String) {
    self.text_fields[index].update_text(new_text);
  }
  
  pub fn _set_textfield_name(&mut self, name: &String, new_name: &String) {
    for text in &mut self.text_fields {
      if text.name_matches(name) {
        text._set_name(new_name);
        break;
      }
    }
  }
  
  pub fn _set_textfield_text_size(&mut self, name: &String, size: Vector2<f32>) {
    for text in &mut self.text_fields {
      if text.name_matches(name) {
        text._set_text_size(size);
        break;
      }
    }
  }
  
  pub fn get_text(&self, name: &String) -> String {
    let mut text = "".to_string();
    for text_field in &self.text_fields {
      if text_field.name_matches(name) {
        text = text_field.get_text();
        break;
      }
    }
    
    text
  }
  
  pub fn _get_textfield_by_index(&mut self, index: usize) -> TextField {
    self.text_fields[index].clone()
  }
  
  pub fn _set_textfield_colour(&mut self, name: &String, text_colour: Vector4<f32>) {
    for text in &mut self.text_fields {
      if text.name_matches(name) {
        text._set_colour(text_colour);
        break;
      }
    }
  }
  
  pub fn _add_textfield(&mut self, name: String, relative_position: Vector2<f32>, size: Vector2<f32>, colour: Vector4<f32>, centered: bool, text: String, font: String) {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.text_fields.push(TextField::new_text_field(name, pos, size, colour, centered, text, font));
  }
  
  
  pub fn _remove_textfield(&mut self, name: &String) {
    for i in 0..self.text_fields.len() {
      if self.text_fields[i].name_matches(name) {
        self.text_fields.remove(i);
        break;
      }
    }
  }
  
  pub fn _duplicate_textfield(&mut self, textfield_to_duplicate: &String, new_textfield_name: &String) {
    for i in 0..self.text_fields.len() {
      if self.text_fields[i].name_matches(textfield_to_duplicate) {
        let mut text = self.text_fields[i].clone();
        text._set_name(new_textfield_name);
        self.text_fields.push(text);
      }
    }
  }
  
  pub fn _textfield_exists(&self, name: &String) -> bool {
    let mut exists = false;
    for text in &self.text_fields {
      if text.name_matches(name) {
        exists = true;
        break;
      }
    }
    exists
  }
  
  pub fn _textfield_name_exists(&self, name: &String) -> bool {
    let mut name_exists = false;
    for text in &self.text_fields {
      if text.name_matches(name) {
        name_exists = true;
        break;
      }
    }
    name_exists
  }
  
  // 
  // BUTTON
  //
  
  pub fn _is_touching_any_button(&self, at_location: Vector2<f32>) -> (bool, usize) {
    let mut is_touching = false;
    let mut i = 0;
    for button in &self.buttons {
      is_touching = button.is_touching(at_location);
      if is_touching {
        break;
      }
      i += 1;
    }
    (is_touching, i)
  }
  
  pub fn _set_button_colour(&mut self, name: &String, pressed: Vector4<f32>, unpressed: Vector4<f32>) {
    for button in &mut self.buttons {
      if button.name_matches(name) {
        button._set_colour(pressed, unpressed);
        break;
      }
    }
  }
  
  pub fn _update_button_location(&mut self, name: &String, pos: Vector2<f32>) {
    let pos = pos;
    for button in &mut self.buttons {
      if button.name_matches(name) {
        button.set_location(pos);
        break;
      }
    }
  }
  
  pub fn _stop_updating_button(&mut self, name: &String) {
    for i in 0..self.buttons.len() {
      if self.buttons[i].name_matches(name) {
        self.buttons[i]._stop_updating();
        break;
      }
    }
  }
  
  pub fn _add_button(&mut self, relative_position: Vector2<f32>, button_size: Vector2<f32>, 
                     text_relative_position: Vector2<f32>, text_size: Vector2<f32>, 
                     name: String, text_colour: Vector4<f32>, pressed_button: Vector4<f32>, 
                     unpressed_button: Vector4<f32>, center_text: bool, text: String, font: String) {
    let pos = (self.position-self.size*0.5) + relative_position;
    self.buttons.push(Button::new_button(name, pos, button_size, text_relative_position, text_size, text_colour, pressed_button, unpressed_button, false, center_text, text, font));
  }
  
  pub fn _remove_button(&mut self, name: &String) {
    for i in 0..self.buttons.len() {
      if self.buttons[i].name_matches(name) {
        self.buttons.remove(i);
        break;
      }
    }
  }
  
  pub fn _duplicate_button(&mut self, button_to_duplicate: &String, new_button_name: &String) {
    for i in 0..self.buttons.len() {
      if self.buttons[i].name_matches(button_to_duplicate) {
        let mut button = self.buttons[i].clone();
        button._set_name(new_button_name);
        self.buttons.push(button);
      }
    }
  }
  
  pub fn _get_button_by_index(&mut self, index: usize) -> Button {
    self.buttons[index].clone()
  }
  
  pub fn get_button_state(&self, button_name: &String) -> bool {
    let mut pressed = false;
    for button in &self.buttons {
      if button.name_matches(button_name) {
        pressed = button.is_pressed();
      }
    }
    pressed
  }
  
  pub fn _set_button_state(&mut self, button_name: &String, new_state: bool) {
    for button in &mut self.buttons {
      if button.name_matches(button_name) {
        if new_state {
          button.toggle_on();
        } else {
          button.toggle_off();
        }
      }
    }
  }
  
  pub fn _button_exists(&self, name: &String) -> bool {
    let mut exists = false;
    for button in &self.buttons {
      if button.name_matches(name) {
        exists = true;
        break;
      }
    }
    exists
  }
  
  // 
  // SLIDER
  //
  
  pub fn _set_slider_value(&mut self, name: &String, new_value: f32) {
    for slider in &mut self.sliders {
      if slider._name_matches(&name) {
        slider.set_value(new_value);
        break;
      }
    }
  }
  
  pub fn _get_slider_value(&self, name: &String) -> Option<f32> {
    let mut value = None;
    for slider in &self.sliders {
      if slider._name_matches(name) {
        value = Some(slider.get_value());
        break;
      }
    }
    value
  }
  
  // 
  // SELECTION
  //
  
  pub fn _get_selected_option_i(&self, selection_set: usize) -> Option<String> {
    self.selections[selection_set].get_selected_option()
  }
  
  pub fn _is_touching_selection_option_i_button_j(&self, selection_set: usize, button_index: usize, against: Vector2<f32>) -> bool {
    self.selections[selection_set]._is_touching_button(button_index, against)
  }
  
  //
  // WIDGET FUNCTIONS
  //
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, keys_pressed_this_frame: &Vec<String>, scroll_delta: f32) {
      for selection in &mut self.selections {
        selection.update(delta_time, mouse_pos, left_mouse);
      }
      for button in &mut self.buttons {
        button.update(delta_time, mouse_pos, left_mouse);
      }
      for dropdown in &mut self.dropdown_boxs {
        dropdown.update(delta_time, mouse_pos, left_mouse, scroll_delta);
      }
      
      if !self.hidden {
        for textfield in &mut self.text_fields {
          textfield.update(delta_time, mouse_pos, left_mouse, keys_pressed_this_frame);
        }
      
        for slider in &mut self.sliders {
          slider.update(delta_time, mouse_pos, left_mouse, scroll_delta);
        }
      }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    if !self.hidden {
      draw_calls.push(DrawCall::draw_coloured(self.position, self.size, self.colour, 90.0));
      for selection in &self.selections {
        selection.draw(draw_calls);
      }
      for button in &self.buttons {
        button.draw(draw_calls);
      }
      for text in &self.text_fields {
        text.draw(draw_calls);
      }
      for slider in &self.sliders {
        slider.draw(draw_calls);
      }
      for dropdown in &self.dropdown_boxs {
        dropdown.draw(draw_calls);
      }
    }
  }
  
  pub fn _get_all(&self) -> (Vec<TextField>, Vec<Button>, Vec<Selection>, Vec<Slider>, Vec<DropdownBox>) {
    let text_fields = self.text_fields.clone();
    let buttons = self.buttons.clone();
    let selections = self.selections.clone();
    let sliders = self.sliders.clone();
    let dropdown_boxs = self.dropdown_boxs.clone();
    (text_fields, buttons, selections, sliders, dropdown_boxs)
  }
  
  
  pub fn _textfields_pop(&mut self) {
    self.text_fields.pop();
  }
  
  pub fn _buttons_pop(&mut self) {
    self.buttons.pop();
  }
  
  pub fn _selections_pop(&mut self) {
    self.selections.pop();
  }
}
