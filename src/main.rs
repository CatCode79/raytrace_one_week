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

    for (i, pixel) in buffer.data.iter_mut().enumerate() {
        let width = i % buffer.width as usize;
        let height = i / buffer.width as usize;

        let r = width as f32 / buffer.width as f32;
        let g = height as f32 / buffer.height as f32;

        *pixel = u32::from_ne_bytes([(255.9999 * r) as u8, (255.9999 * g) as u8, 0_u8, 255_u8]);
    }
}
