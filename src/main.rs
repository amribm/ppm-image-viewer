extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::Duration;

pub fn main() {
    let file = File::open("one-peice.ppm").unwrap();
    let mut reader = BufReader::new(file);

    {
        let mut buf = String::new();
        let _ = reader.read_line(&mut buf);
    }

    let mut height = 0;
    let mut width = 0;
    {
        let mut buf = String::new();
        let _ = reader.read_line(&mut buf);
        println!(" width and height : {}", buf);
        let splitted: Vec<u32> = buf
            .trim()
            .split(" ")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        println!(" width and height splitted : {:?}", splitted);
        width = splitted[0];
        height = splitted[1];
    }

    {
        let mut buf = String::new();
        let _ = reader.read_line(&mut buf);
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    for y in 0..height {
        for x in 0..width {
            let mut rgb: [u8; 3] = [0, 0, 0];
            let _ = reader.read_exact(&mut rgb);
            canvas.set_draw_color(Color::RGB(rgb[0], rgb[1], rgb[2]));
            canvas
                .fill_rect(Rect::new(x as i32, y as i32, 1, 1))
                .unwrap();
        }
    }
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
