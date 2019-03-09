extern crate winit;
extern crate maat_graphics;
extern crate maat_input_handler;
extern crate cgmath;
extern crate rand;

mod modules;

use crate::modules::scenes::Scene;
use crate::modules::scenes::LoadScreen;

use maat_graphics::graphics::CoreRender;
use maat_graphics::CoreMaat;
use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector4};

use std::time;

const VERSION: &str = "0.1.0";

fn benchmark(draw_calls: &mut Vec<DrawCall>, dimensions: [f32; 2]) {
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions[0] - 64.0, 5.0), 
                                           Vector2::new(128.0, 128.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "v".to_string() + VERSION, 
                                           "Arial".to_string()));
                                           /*
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(10.0, dimensions[1]*0.4), 
                                           Vector2::new(256.0, 256.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
 "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ123456789".to_string(), 
                                           "Arial".to_string()));
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(10.0, dimensions[1]*0.3), 
                                           Vector2::new(256.0, 256.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
 "`~!@#$%^&&*()_-+=[{}]\\|;:\"\',<.>/?'".to_string(), 
                                           "Arial".to_string()));*/
}

fn fps_overlay(draw_calls: &mut Vec<DrawCall>, dimensions: [f32; 2], fps: f64, ms: f64) {
  let  mut fps = fps.to_string();
  fps.truncate(6);
  let mut ms = ms.to_string();
  ms.truncate(4);
  
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(64.0, dimensions[1]-32.0), 
                                           Vector2::new(128.0, 128.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "fps: ".to_string() + &fps, 
                                           "Arial".to_string()));
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(64.0, dimensions[1]-64.0), 
                                           Vector2::new(128.0, 128.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "".to_string() + &ms + &" ms/frame".to_string(), 
                                           "Arial".to_string()));
}

fn main() {
  let mut graphics = CoreMaat::new("Delinquent Food".to_string(), (0 as u32) << 22 | (0 as u32) << 12 | (6 as u32), 1280.0, 720.0, true);
  
  graphics.preload_font(String::from("Arial"),
                        String::from("./resources/Fonts/Purisa.png"),
                        include_bytes!("../resources/Fonts/Purisa.fnt"));
  graphics.preload_texture(String::from("Logo"), 
                           String::from("./resources/Textures/Logo.png"));
  
  graphics.add_model("Lance".to_string(), "./windys-modeling-agency/Unfinished/Lance.glb".to_string());
  graphics.add_model("Chair".to_string(), "./windys-modeling-agency/Unfinished/chair-1stattempt.glb".to_string());
  graphics.add_model("Tower".to_string(), "./windys-modeling-agency/Unfinished/TowerStart.glb".to_string());
  graphics.add_model("Floor".to_string(), "./resources/Models/Floor/Floor.glb".to_string());
  graphics.add_model("FloorPath".to_string(), "./resources/Models/Floor/FloorPath.glb".to_string());
  
  graphics.add_model("Hexagon".to_string(), "./windys-modeling-agency/Unfinished/hexagon.glb".to_string());
  graphics.add_model("BlueHexagon".to_string(), "./windys-modeling-agency/Unfinished/Bluehexagon.glb".to_string());
  graphics.add_model("GreenHexagon".to_string(), "./windys-modeling-agency/Unfinished/GreenHexagon.glb".to_string());
  graphics.add_model("PurpleHexagon".to_string(), "./windys-modeling-agency/Unfinished/PurpleHexagon.glb".to_string());
  graphics.add_model("RedHexagon".to_string(), "./windys-modeling-agency/Unfinished/RedHexagon.glb".to_string());
  graphics.add_model("Bombard".to_string(), "./windys-modeling-agency/Unfinished/Bombard.glb".to_string());
  
  // Towers
  graphics.add_model("Fridge".to_string(), "./windys-modeling-agency/Unfinished/Fridge.glb".to_string());
  graphics.add_model("Dishwasher".to_string(), "./windys-modeling-agency/Unfinished/TowerStart.glb".to_string());
  
  // Weapons
  graphics.add_model("Spoon".to_string(), "./windys-modeling-agency/Unfinished/Spoon.glb".to_string());
  graphics.add_model("Plate".to_string(), "./windys-modeling-agency/Unfinished/Plate.glb".to_string());
  
  // Enemies
  graphics.add_model("Strawberry".to_string(), "./windys-modeling-agency/Unfinished/Strawberry.glb".to_string());
  graphics.add_model("Banana".to_string(), "./windys-modeling-agency/Unfinished/Banana.glb".to_string());
  
  graphics.create_instance_buffer("EntityBuffer".to_string());
  
  graphics.load_shaders();
  graphics.init();
  
  graphics.set_clear_colour(0.2, 0.2, 0.2, 1.0);
  
  let mut game: Box<Scene> = Box::new(LoadScreen::new());
  
  let mut draw_calls: Vec<DrawCall> = Vec::with_capacity(100);
  
  let mut delta_time;
  let mut last_time = time::Instant::now();
  
  let mut done = false;
  let mut dimensions;
  let dpi = 1.0;
  let mut dpi_changed = false;
  
  let mut frame_counter = 0;
  let mut fps_timer = 0.0;
  let mut last_fps = 0.0;
  let mut ms = 0.0;
  
  loop {
    delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
    last_time = time::Instant::now();
    
    frame_counter += 1;
    fps_timer += delta_time;
    if fps_timer > 1.0 {
      last_fps = frame_counter as f64 * (1.0/fps_timer);
      ms = (fps_timer*1000.0) / frame_counter as f64;
      fps_timer = 0.0;
      frame_counter = 0;
    }
    
    dimensions = {
      let dim = graphics.get_dimensions();
      [dim.width as f32 * dpi, dim.height as f32  * dpi]
    };
    
    if game.scene_finished() {
      game = game.future_scene(Vector2::new(dimensions[0], dimensions[1]));
    }
    
    game.set_window_dimensions(Vector2::new(dimensions[0], dimensions[1]));
    
    game.update(delta_time as f32);
    
    game.draw(&mut draw_calls);
    draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions[0] - 256.0, 100.0), 
                                           Vector2::new(128.0, 128.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           (draw_calls.len() + 1).to_string(), 
                                           "Arial".to_string()));
    benchmark(&mut draw_calls, dimensions);
    fps_overlay(&mut draw_calls, dimensions, last_fps, ms);
    
    let model_details = graphics.pre_draw();
    graphics.draw(&draw_calls, delta_time as f32);
    graphics.post_draw();
    
    draw_calls.clear();
    
    game.reset_scroll_value();
    for (reference, size) in &model_details {
      game.add_model_size(reference.to_string(), *size);
    }
    
    let mut resized = false;
    
    let _height = graphics.get_dimensions().height as f32;
    graphics.get_events().poll_events(|ev| {
      match ev {
        winit::Event::WindowEvent{ event, .. } => {
          match event {
            winit::WindowEvent::Resized(_new_size) => {
              resized = true;
            },
            winit::WindowEvent::CursorMoved{device_id: _, position, modifiers: _} => {
             // let mut _mouse_coords = [0.0, 0.0];
             // _mouse_coords[0] = (position.x as f32 / dpi_scale) as f32;
             // _mouse_coords[1] =  height - (position.y as f32 / dpi_scale) as f32;
              game.set_mouse_position(Vector2::new(position.x as f32, dimensions[1] / dpi - position.y as f32));
            },
            winit::WindowEvent::CloseRequested => {
              done = true;
            },
            winit::WindowEvent::HiDpiFactorChanged(new_dpi) => {
              println!("Dpi Changed: {}", new_dpi);
           //   dpi = new_dpi as f32;
              dpi_changed = true;
            },
            _ => {
              if game.handle_input(event) {
                done = true;
              }
            }
          }
        },
        _ => {},
      }
    });
    
    if dpi_changed {
      dpi_changed = false;
    }
    
    if resized {
      graphics.screen_resized();
    }
    
    if done { break; }
  }
  
  println!("Game Loop ended");
}
