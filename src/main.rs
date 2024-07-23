use minifb::{Key, Window, WindowOptions, Result};
use std::time::Duration;
mod framebuffer;
mod disenos;
use framebuffer::Framebuffer;
use disenos::{ten_cell_row, exploder, small_exploder, glider, spaceship};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn initialize_pattern(framebuffer: &mut Framebuffer) {
    // // Aquí define las posiciones de los patrones específicos
    // ten_cell_row(framebuffer, 0, 5);
    // ten_cell_row(framebuffer, 10, 5);
    // ten_cell_row(framebuffer, 20, 5);
    // ten_cell_row(framebuffer, 30, 5);
    // ten_cell_row(framebuffer, 40, 5);
    // ten_cell_row(framebuffer, 50, 5);
    // ten_cell_row(framebuffer, 60, 5);
    // ten_cell_row(framebuffer, 70, 5);
    // ten_cell_row(framebuffer, 80, 5);
    // ten_cell_row(framebuffer, 90, 5);
    
    
    ten_cell_row(framebuffer, 5, 0);
    ten_cell_row(framebuffer, 15, 0);
    ten_cell_row(framebuffer, 25, 0);
    ten_cell_row(framebuffer, 35, 0);
    ten_cell_row(framebuffer, 45, 0);
    ten_cell_row(framebuffer, 55, 0);
    ten_cell_row(framebuffer, 65, 0);
    ten_cell_row(framebuffer, 75, 0);
    ten_cell_row(framebuffer, 85, 0);
    ten_cell_row(framebuffer, 95, 0);

    // small_exploder(framebuffer, 50, 50);
    // small_exploder(framebuffer, 60, 60);
    // small_exploder(framebuffer, 40, 40);
    // small_exploder(framebuffer, 60, 40);
    // small_exploder(framebuffer, 40, 60);
    // small_exploder(framebuffer, 70, 70);
    // small_exploder(framebuffer, 30, 30);
    // small_exploder(framebuffer, 70, 30);
    // small_exploder(framebuffer, 30, 70);
    // small_exploder(framebuffer, 80, 80);
    // small_exploder(framebuffer, 20, 20);
    // small_exploder(framebuffer, 80, 20);
    // small_exploder(framebuffer, 20, 80);

    // // glider(framebuffer, 0, 10);
    // // glider(framebuffer, 70, 50);
    // exploder(framebuffer, 10, 35);
    // exploder(framebuffer, 10, 65);
    // exploder(framebuffer, 90, 35);
    // exploder(framebuffer, 90, 65);
    // exploder(framebuffer, 20, 35);
    // exploder(framebuffer, 20, 65);
    // exploder(framebuffer, 80, 35);
    // exploder(framebuffer, 80, 65);
    // exploder(framebuffer, 15, 50);
    // exploder(framebuffer, 85, 50);
    // spaceship(framebuffer, 0, 50);

}

fn count_live_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> usize {
    let mut count = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (dx, dy) in directions.iter() {
        let nx = (x as isize + dx) % WIDTH as isize;
        let ny = (y as isize + dy) % HEIGHT as isize;

        let nx = if nx < 0 { nx + WIDTH as isize } else { nx } as usize;
        let ny = if ny < 0 { ny + HEIGHT as isize } else { ny } as usize;

        if framebuffer.get(nx, ny) == 0xFFDDDD {
            count += 1;
        }
    }
    count
}

fn render(framebuffer: &mut Framebuffer) {
    let mut new_framebuffer = framebuffer.clone();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let alive = framebuffer.get(x, y) == 0xFFDDDD;
            let neighbors = count_live_neighbors(framebuffer, x, y);

            let new_state = match (alive, neighbors) {
                (true, 2) | (true, 3) => 0xFFDDDD, // Sobrevive
                (true, _) => 0x333355, // Muere
                (false, 3) => 0xFFDDDD, // Nace
                _ => 0x333355, // Permanece muerto
            };

            new_framebuffer.set_color(x, y, new_state);
        }
    }

    *framebuffer = new_framebuffer;
}

fn main() -> Result<()> {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = WIDTH;
    let framebuffer_height = HEIGHT;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    initialize_pattern(&mut framebuffer);

    let mut window = Window::new(
        "Game of Life - Conway",
        window_width,
        window_height,
        WindowOptions::default(),
    )?;

    while window.is_open() {
        if window.is_key_down(Key::A) {
            break;
        }

        render(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.get_buffer(), framebuffer_width, framebuffer_height)?;

        std::thread::sleep(frame_delay);
    }

    Ok(())
}