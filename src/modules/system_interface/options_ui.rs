use maat_graphics::DrawCall;

use crate::modules::system_interface::Widget;

use cgmath::Vector2;
use cgmath::Vector4;

const MAIN_WINDOW_INDEX: usize = 0;

const MAIN_WINDOW_NAME: &str = "MainWindow"; 
const APPLY_BUTTON_NAME: &str = "Apply";
const RETURN_BUTTON_NAME: &str = "Return";
const LOWER_RESOLUTION_BUTTON: &str = "LowerRes";
const HIGHER_RESOLUTION_BUTTON: &str = "HigherRes";
const RESOLUTION_NAME: &str = "Resolution";
const VSYNC_BUTTON: &str = "Vsync";
const DPI_BUTTON: &str = "Dpi";
const CURRENT_DPI_NAME: &str = "CurrentDpi";
const LOWER_DPI_BUTTON: &str = "LowerDpi";
const HIGHER_DPI_BUTTON: &str = "HigherDpi";
const FULLSCREEN_BUTTON: &str = "Fullscreen";

pub struct OptionsUi {
  apply_settings: bool,
  hidden: bool,
  available_resolutions: Vec<Vector2<i32>>,
  current_resolution_index: usize,
  widgets: Vec<Widget>
}

impl OptionsUi {
  pub fn new(window_size: Vector2<f32>) -> OptionsUi {
    let mut widgets = Vec::new();
    
    let mut resolutions = vec!(Vector2::new(800, 600), // 4:3
                           Vector2::new(960, 720), // 4:3
                           Vector2::new(1024, 768), // 4:3
                           Vector2::new(1152, 864), // 4:3
                           Vector2::new(1280, 720), // 16:9
                           Vector2::new(1280, 800), // 16:10
                           Vector2::new(1280, 960), //4:3
                           Vector2::new(1280, 1024), // 5:4
                           Vector2::new(1360, 768), // 16:9
                           Vector2::new(1600, 900), // 16:9
                           Vector2::new(1680, 1050), // 16:10
                           Vector2::new(1920, 1080), // 16:9
                           Vector2::new(2560, 1080) // 21:9
                           );
    let mut res_index: i32 = -1;
    for i in 0..resolutions.len() {
      if resolutions[i].x == window_size.x as i32 && resolutions[i].y == window_size.y as i32 {
        res_index = i as i32;
        break;
      }
    }
    if res_index == -1 {
      for i in 0..resolutions.len() {
        if resolutions[i].x > window_size.x as i32 {
          let mut j = i;
          if j > 0 {
            j-=1;
          }
          resolutions.insert(j, Vector2::new(window_size.x as i32, window_size.y as i32));
          res_index = i as i32-1;
          break;
        }
      }
    }
    
    let widget_width = 400.0;
    let widget_height = 400.0;
    
    let button_width = 128.0;
    let button_height = 32.0;
    
    let text_location = widget_height-75.0;
    let text_height = 33.0;
    let text_offset = text_height*1.5;
    
    let text_colour = Vector4::new(0.0, 0.0, 0.0, 1.0);
    let button_colour = Vector4::new(0.5019, 0.749, 1.0, 1.0);
    let no_colour = Vector4::new(0.0, 0.0, 0.0, 0.0);
    
    let res_up_down_size = Vector2::new(20.0, 32.0);
    
    widgets.push(Widget::new(MAIN_WINDOW_NAME.to_string(), window_size * 0.5, Vector2::new(widget_width, widget_height), Vector4::new(0.337254902, 0.662745098, 0.788235294, 1.0))
                 .start_hidden()
                 .with_button(Vector2::new(widget_width-25.0-button_width*0.5, button_height*1.5),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             APPLY_BUTTON_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(1.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Apply".to_string(), "Arial".to_string())
                 .with_button(Vector2::new(25.0+button_width*0.5, button_height*1.5),
                             Vector2::new(button_width, button_height),
                             Vector2::new(button_width*0.5, button_height*0.33), 
                             Vector2::new(128.0, 128.0),
                             RETURN_BUTTON_NAME.to_string(), 
                             text_colour, 
                             Vector4::new(1.0, 0.0, 0.0, 1.0),
                             button_colour,
                             true, "Return".to_string(), "Arial".to_string())
                 .with_text_field("PlainTextFullscreen".to_string(),
                                  Vector2::new(25.0, text_location), 
                                  Vector2::new(256.0, 256.0), 
                                  Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                  "fullscreen:".to_string(), "Arial".to_string())
                 .with_toggle_button_textured(Vector2::new(widget_width-125.0, text_location+15.0), 
                                               Vector2::new(25.0, 25.0), FULLSCREEN_BUTTON.to_string(), 
                                               "Tickbox_ticked".to_string(), 
                                               "Tickbox_unticked".to_string())
                 .with_text_field("PlainTextResolution".to_string(), 
                                  Vector2::new(25.0, text_location-text_offset), 
                                  Vector2::new(256.0, 256.0), 
                                  Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                  "Resolution:".to_string(), "Arial".to_string())
                 .with_button(Vector2::new(widget_width-200.0, text_location-text_offset+10.0),
                             res_up_down_size,
                             Vector2::new(res_up_down_size.x*0.5, res_up_down_size.y*0.1), 
                             Vector2::new(256.0, 256.0),
                             LOWER_RESOLUTION_BUTTON.to_string(), 
                             text_colour, 
                             no_colour,
                             no_colour,
                             true, "<".to_string(), "Arial".to_string())
                  .with_button(Vector2::new(widget_width-50.0, text_location-text_offset+10.0),
                             res_up_down_size,
                             Vector2::new(res_up_down_size.x*0.5, res_up_down_size.y*0.1), 
                             Vector2::new(256.0, 256.0),
                             HIGHER_RESOLUTION_BUTTON.to_string(), 
                             text_colour, 
                             no_colour,
                             no_colour,
                             true, ">".to_string(), "Arial".to_string())
                  .with_text_field_centered(RESOLUTION_NAME.to_string(), 
                                   Vector2::new(widget_width-125.0, text_location-text_offset-5.0), 
                                   Vector2::new(256.0, 256.0), Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                   (window_size.x.to_string().to_owned() + "x" + &window_size.y.to_string()).to_string(), 
                                   "Arial".to_string())
                  .with_text_field("PlainTextVsync".to_string(), 
                                  Vector2::new(25.0, text_location-text_offset*2.0), 
                                  Vector2::new(256.0, 256.0), 
                                  Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                  "Vysnc:".to_string(), "Arial".to_string())
                  .with_toggle_button_textured(Vector2::new(widget_width-125.0, text_location-text_offset*2.0+15.0), 
                                               Vector2::new(25.0, 25.0), VSYNC_BUTTON.to_string(), 
                                               "Tickbox_ticked".to_string(), 
                                               "Tickbox_unticked".to_string())
                  .with_text_field("PlainTextDpi".to_string(), 
                                  Vector2::new(25.0, text_location-text_offset*3.0), 
                                  Vector2::new(256.0, 256.0), 
                                  Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                  "Force Dpi:".to_string(), "Arial".to_string())
                  .with_toggle_button_textured(Vector2::new(190.0, text_location-text_offset*3.0+15.0), 
                                               Vector2::new(25.0, 25.0), DPI_BUTTON.to_string(), 
                                               "Tickbox_ticked".to_string(), 
                                               "Tickbox_unticked".to_string())
                  .with_text_field(CURRENT_DPI_NAME.to_string(), 
                                  Vector2::new(280.0, text_location-text_offset*3.0-15.0), 
                                  Vector2::new(256.0, 256.0), 
                                  Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                  "1".to_string(), "Arial".to_string())
                 .with_button(Vector2::new(250.0, text_location-text_offset*3.0),
                             res_up_down_size,
                             Vector2::new(res_up_down_size.x*0.5,
                                          res_up_down_size.y*0.1), 
                             Vector2::new(256.0, 256.0),
                             LOWER_DPI_BUTTON.to_string(), 
                             text_colour, 
                             no_colour,
                             no_colour,
                             true, "<".to_string(), "Arial".to_string())
                  .with_button(Vector2::new(320.0, text_location-text_offset*3.0),
                             res_up_down_size,
                             Vector2::new(res_up_down_size.x*0.5,
                                          res_up_down_size.y*0.1), 
                             Vector2::new(256.0, 256.0),
                             HIGHER_DPI_BUTTON.to_string(), 
                             text_colour, 
                             no_colour,
                             no_colour,
                             true, ">".to_string(), "Arial".to_string())
                );
    
    OptionsUi {
      apply_settings: false,
      hidden: true,
      available_resolutions: resolutions,
      current_resolution_index: res_index as usize,
      widgets: widgets,
    }
  }
  
  pub fn show(&mut self) {
    for widget in &mut self.widgets {
      widget.show();
    }
    self.hidden = false;
  }
  
  pub fn hide(&mut self) {
    for widget in &mut self.widgets {
      widget.hide();
    }
    self.hidden = true;
  }
  
  pub fn is_hidden(&self) -> bool {
    self.hidden
  }
  
  pub fn update(&mut self, delta_time: f32, mouse_pos: Vector2<f32>, left_mouse: bool, keys_pressed_this_frame: &Vec<String>, scroll_delta: f32) {
    self.apply_settings = false;
    for widget in &mut self.widgets {
      widget.update(delta_time, mouse_pos, left_mouse, keys_pressed_this_frame, scroll_delta);
    }
    
    if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&RETURN_BUTTON_NAME.to_string()) {
      println!("Hiding menu");
      self.hide();
    }
    
    if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&APPLY_BUTTON_NAME.to_string()) {
      self.apply_settings = true;
    }
    
    
    if !self.widgets[MAIN_WINDOW_INDEX].get_button_state(&FULLSCREEN_BUTTON.to_string()) {
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&LOWER_RESOLUTION_BUTTON.to_string()) {
        if self.current_resolution_index == 0 {
          self.current_resolution_index = self.available_resolutions.len()-1;
        } else {
          self.current_resolution_index -= 1;
        }
        
        let i = self.current_resolution_index;
        let width = self.available_resolutions[i].x;
        let height = self.available_resolutions[i].y;
        self.widgets[MAIN_WINDOW_INDEX].update_text_field(&RESOLUTION_NAME.to_string(), (width.to_string().to_owned() + "x" + &height.to_string()).to_string());
      }
      
      if self.widgets[MAIN_WINDOW_INDEX].get_button_state(&HIGHER_RESOLUTION_BUTTON.to_string()) {
        if self.current_resolution_index == self.available_resolutions.len()-1 {
          self.current_resolution_index = 0;
        } else {
          self.current_resolution_index += 1;
        }
        
        let i = self.current_resolution_index;
        let width = self.available_resolutions[i].x;
        let height = self.available_resolutions[i].y;
        self.widgets[MAIN_WINDOW_INDEX].update_text_field(&RESOLUTION_NAME.to_string(), (width.to_string().to_owned() + "x" + &height.to_string()).to_string());
      }
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    for widget in &self.widgets {
      widget.draw(draw_calls);
    }
    
    if self.apply_settings {
      // do stuff
      draw_calls.push(DrawCall::change_dpi(self.widgets[MAIN_WINDOW_INDEX].get_text(&CURRENT_DPI_NAME.to_string()).parse::<f32>().unwrap()));
      draw_calls.push(DrawCall::enable_dpi(self.widgets[MAIN_WINDOW_INDEX].get_button_state(&DPI_BUTTON.to_string())));
      draw_calls.push(DrawCall::enable_vsync(self.widgets[MAIN_WINDOW_INDEX].get_button_state(&VSYNC_BUTTON.to_string())));
      let fullscreen = self.widgets[MAIN_WINDOW_INDEX].get_button_state(&FULLSCREEN_BUTTON.to_string());
      draw_calls.push(DrawCall::enable_fullscreen(fullscreen));
      if !fullscreen {
        draw_calls.push(DrawCall::change_resolution(self.available_resolutions[self.current_resolution_index]));
      }
      println!("settings applied!");
    }
  }
}
