use glam::UVec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

mod color;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("raytrace_one_week", 1280, 720)
        .resizable()
        //.maximized()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_size = {
        let size = window.size();
        UVec2::new(size.0 / 2, size.1 / 2)
    };

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();
    let mut texture = {
        let t = creator
            .create_texture_target(PixelFormatEnum::RGB888, texture_size.x, texture_size.y)
            .map_err(|e| e.to_string())?;
        t
    };

    let mut image = vec![0_u8; (texture_size.x * texture_size.y * 3) as usize];

    let mut x = 0;
    while x < texture_size.x {
        let mut y = 0;
        while y < texture_size.y {
            let r = x as f32 / texture_size.x as f32;
            let g = y as f32 / texture_size.y as f32;
            let b = 0_f32;

            image.insert((x * y  + x) as usize, (255.9999 * r) as u8);
            image.insert((x * y  + x) as usize, (255.9999 * g) as u8);
            image.insert((x * y  + x) as usize, (255.9999 * b) as u8);

            y += 1;
        }
        x += 1;
    }

    texture.update(None, image.as_slice(), (texture_size.x * 3) as usize)
        .map_err(|e| e.to_string())?;
    canvas.clear();

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

        println!("passa");
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}
