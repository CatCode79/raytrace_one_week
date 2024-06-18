use glam::UVec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("raytrace_one_week", 1280, 720)
        .resizable()
        //.maximized()
        .build()
        .map_err(|e| e.to_string())?;

    let (window_width, window_height) = {
        let size = window.size();
        (size.0, size.1)
    };

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let mut x: i32 = 0;
    while x < window_width as i32 {
        let mut y: i32 = 0;
        while y < window_height as i32 {
            let r = x as f32 / window_width as f32;
            let g = y as f32 / window_height as f32;
            let b = 0_f32;

            // TODO: It's probably faster to use a texture
            let color = Color::RGB((255.9999 * r) as u8, (255.9999 * g) as u8, (255.9999 * b) as u8);
            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(x, y))?;

            y += 1;
        }
        x += 1;
    }

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }

        canvas.present();
    }

    Ok(())
}
