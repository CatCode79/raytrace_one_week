mod buffer;
mod renderer;

use crate::buffer::Buffer;
use crate::renderer::Renderer;

use fenestella::mapping::InputMapping;
use fenestella::window::{Event, Window};

// The additional values are used to obtain a padding of 256 relative to the buffer.
const WIDTH: u16 = 1280 + 16;
const HEIGHT: u16 = 720 + 39;

fn main() -> Result<(), String> {
    let mut buffer = Buffer {
        width: WIDTH,
        height: HEIGHT,
        data: vec![0_u32; WIDTH as usize * HEIGHT as usize],
    };

    let input_mapping = InputMapping::new();
    let mut window = Window::new("Raytrace One Week".to_string(), WIDTH, HEIGHT, input_mapping)?;

    let mut renderer = Renderer::new(&window)?;

    loop {
        let events = window.process_events();
        for event in events {
            match event {
                Event::Resize { width, height } => {
                    buffer.width = width.get();
                    buffer.height = height.get();
                    buffer.data = vec![0_u32; buffer.width as usize * buffer.height as usize];

                    renderer.resize(width, height);
                }
            }
        }

        draw(&mut buffer);
        renderer.update(&buffer)?;

        renderer.present();
        profiling::finish_frame!();
    }

    Ok(())
}

fn draw(buffer: &mut Buffer) {
    profiling::scope!("render");

    let mut y = 0;
    while y < buffer.height {
        let mut x = 0;
        while x < buffer.width {
            let r = x as f32 / WIDTH as f32;
            let g = y as f32 / HEIGHT as f32;
            let b = 0_f32;

            let r = (255.9999 * r) as u32;
            let g = (255.9999 * g) as u32;
            let b = (255.9999 * b) as u32;
            let color = r << 16 | g << 8 | b;

            buffer.data[x as usize * y as usize + x as usize] = color;

            x += 1;
        }
        y += 1;
    }
}
