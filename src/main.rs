#![feature(duration_zero)]
#[macro_use]
extern crate bitflags;
extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::{thread};
use std::time::{Duration, Instant};


#[path = "CPU6502.rs"] mod CPU6502;

fn main() -> Result<(), String> {
    let mut cpu = CPU6502::CPU6502::new();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut step_mode = false;
    let mut time_left: i64 = 0;
    let mut start_time = Instant::now();
    let mut new_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::N), ..} => {
                    if step_mode {
                        cpu.clock();
                        while cpu.cycles_to_wait != 0 {
                            cpu.clock();
                        }
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    step_mode = !step_mode;
                },
                _ => {}
            }
        }

        if !step_mode {
            new_time = Instant::now();
            let time_taken = new_time.duration_since(start_time);

            if time_left >= 0 {
                time_left -= time_taken.subsec_nanos() as i64;
            } else {
                time_left = Duration::from_nanos(((1.0 / 60.0) * 1_000_000.0) as u64).subsec_nanos() as i64 - time_taken.subsec_nanos() as i64;
                
                let frame_done: bool = cpu.clock();
                if frame_done {
                    canvas.clear();
                    texture.update(None, cpu.get_frame_buffer().as_ref(), 0);
                    canvas.copy(&texture, None, None)?;
                    canvas.present();
                }
            }

            start_time = new_time;
        }
    }

    Ok(()) 
}
