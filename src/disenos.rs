use crate::framebuffer::Framebuffer;

pub fn ten_cell_row(framebuffer: &mut Framebuffer, start_x: usize, start_y: usize) {
    let pattern = [
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), 
        (5, 0), (6, 0), (7, 0), (8, 0), (9, 0)
    ];
    draw_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn exploder(framebuffer: &mut Framebuffer, start_x: usize, start_y: usize) {
    let pattern = [
        (0, 0), (1, 0), (2, 0), 
        (0, 1), (2, 1), 
        (0, 2), (1, 2), (2, 2)
    ];
    draw_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn small_exploder(framebuffer: &mut Framebuffer, start_x: usize, start_y: usize) {
    let pattern = [
        (1, 0), 
        (0, 1), (1, 1), (2, 1), 
        (0, 2), (2, 2), 
        (1, 3)
    ];
    draw_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn glider(framebuffer: &mut Framebuffer, start_x: usize, start_y: usize) {
    let pattern = [
        (1, 0), 
        (2, 1), 
        (0, 2), (1, 2), (2, 2)
    ];
    draw_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn spaceship(framebuffer: &mut Framebuffer, start_x: usize, start_y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0), 
        (0, 1), 
        (4, 1), 
        (4, 2), 
        (0, 3), (3, 3)
    ];
    draw_pattern(framebuffer, start_x, start_y, &pattern);
}

fn draw_pattern(framebuffer: &mut Framebuffer, start_x: usize, start_y: usize, pattern: &[(usize, usize)]) {
    for &(x, y) in pattern.iter() {
        framebuffer.point(start_x + x, start_y + y);
    }
}
