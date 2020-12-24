#![windows_subsystem = "windows"]

extern crate sdl2;

use rand::{distributions::Uniform, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

use sdl2::gfx::primitives::DrawRenderer;

fn new_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);

    (0..50).map(|_| rng.sample(&range)).collect()
}

fn bubble_sort_step(mut arr: Vec<i32>, mut idx: usize) -> (Vec<i32>, usize) {
    if idx == 49 {
        idx = 0;
    }

    if arr[idx] > arr[idx + 1] {
        arr.swap(idx, idx + 1)
    }
    idx += 1;
    (arr, idx)
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Sorting Algorithms", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(51, 51, 51));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut sorting = false;
    let mut idx = 0;
    let mut arr: Vec<i32> = new_array();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if x > 250 && x < 330 && y > 50 && y < 65 {
                        arr = new_array();
                        idx = 0;
                        sorting = false;
                    } else if x > 470 && x < 565 && y > 50 && y < 65 {
                        sorting = true;
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(51, 51, 51));
        canvas.clear();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        canvas.box_(250, 50, 330, 65, Color::RGB(255, 255, 255))?;
        canvas.string(255, 55, "New Array", Color::RGB(0, 0, 0))?;

        canvas.box_(470, 50, 565, 65, Color::RGB(255, 255, 255))?;
        canvas.string(475, 55, "Bubble Sort", Color::RGB(0, 0, 0))?;

        for (i, item) in arr.iter().enumerate() {
            let i = i as i16;
            let item = *item as i16;

            if i == idx as i16 || i == idx as i16 + 1 {
                canvas.rounded_box(
                    100 + i * 12,
                    500,
                    105 + i * 12,
                    500 - 10 - item * 18,
                    2,
                    Color::RGB(255, 157, 0),
                )?;
            } else {
                canvas.rounded_box(
                    100 + i * 12,
                    500,
                    105 + i * 12,
                    500 - 10 - item * 18,
                    2,
                    Color::RGB(97, 218, 251),
                )?;
            }
        }

        if sorting {
            let (arr2, idx2) = bubble_sort_step(arr, idx);
            arr = arr2;
            idx = idx2;
        }

        canvas.present();
    }
    Ok(())
}
