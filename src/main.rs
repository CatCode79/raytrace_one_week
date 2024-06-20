use minifb::{Key, Window, WindowOptions};

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 720;

fn main() -> Result<(), String> {
    let mut buffer: Vec<u32> = vec![0; WIDTH as usize * HEIGHT as usize];

    let mut window = {
        let options = WindowOptions {
            resize: true,
            ..Default::default()
        };
        Window::new(
            "Raytracing in one week",
            WIDTH as usize,
            HEIGHT as usize,
            options,
        )
    }.unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render(&mut buffer);

        window
            .update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize)
            .unwrap();

        profiling::finish_frame!();
    }

    Ok(())
}

fn render(buffer: &mut Vec<u32>) {
    profiling::scope!("render");

    let mut y = 0;
    while y < HEIGHT {
        let mut x = 0;
        while x < WIDTH {
            let r = x as f32 / WIDTH as f32;
            let g = y as f32 / HEIGHT as f32;
            let b = 0_f32;

            let r = (255.9999 * r) as u32;
            let g = (255.9999 * g) as u32;
            let b = (255.9999 * b) as u32;
            let color = r << 16 | g << 8 | b;

            buffer[x as usize * y as usize + x as usize] = color;

            x += 1;
        }
        y += 1;
    }
}
