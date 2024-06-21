use fenestella::mapping::InputMapping;
use fenestella::window::{Event, Window};

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 720;

fn main() -> Result<(), String> {
    let mut buffer_width = WIDTH as usize;
    let mut buffer_height = HEIGHT as usize;
    let mut buffer = vec![0_u32; buffer_width * buffer_height];

    let input_mapping = InputMapping::new();
    let mut window = Window::new("Raytrace One Week".to_string(), WIDTH, HEIGHT, input_mapping)?;

    loop {
        let events = window.process_events();
        for event in events {
            match event {
                Event::Resize { mut width, mut height } => {
                    buffer_width = width.get() as usize;
                    buffer_height = height.get() as usize;
                    buffer = vec![0_u32; buffer_width * buffer_height];
                }
            }
        }

        draw(&mut buffer, buffer_width, buffer_height);

        profiling::finish_frame!();
    }

    Ok(())
}

fn draw(buffer: &mut Vec<u32>, width: usize, height: usize) {
    profiling::scope!("render");

    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let r = x as f32 / WIDTH as f32;
            let g = y as f32 / HEIGHT as f32;
            let b = 0_f32;

            let r = (255.9999 * r) as u32;
            let g = (255.9999 * g) as u32;
            let b = (255.9999 * b) as u32;
            let color = r << 16 | g << 8 | b;

            buffer[x * y + x] = color;

            x += 1;
        }
        y += 1;
    }
}
