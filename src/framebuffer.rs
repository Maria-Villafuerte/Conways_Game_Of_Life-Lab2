// framebuffer.rs
#[derive(Clone)]
pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0x333355; width * height]; // Color de fondo predeterminado
        let background_color = 0x333355;
        let current_color = 0xFFDDDD;
        Framebuffer { width, height, buffer, background_color, current_color }
    }

    pub fn get_color_at(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index]
        } else {
            self.background_color // Color de fondo predeterminado si está fuera de los límites
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn clear(&mut self) {
        self.set_background_color(self.background_color);
    }

    pub fn point(&mut self, x: usize, y: usize) {
        self.set_color(x, y, self.current_color);
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}
