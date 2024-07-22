// framebuffer.rs
pub struct Framebuffer {
  pub width: usize,
  pub height: usize,
  pub buffer: Vec<u32>, // Hacemos el buffer público
  pub background_color: u32,
  pub current_color: u32,
}

impl Framebuffer {
  // Crea un nuevo framebuffer con el tamaño dado
  pub fn new(width: usize, height: usize) -> Self {
      Self {
          width,
          height,
          buffer: vec![0; width * height],
          background_color: 0x000000, // Negro por defecto
          current_color: 0xFFFFFF, // Blanco por defecto
      }
  }

  // Establece el color de fondo
  pub fn set_background_color(&mut self, color: u32) {
      self.background_color = color;
  }

  // Establece el color actual para dibujar
  pub fn set_current_color(&mut self, color: u32) {
      self.current_color = color;
  }

  // Limpia el framebuffer con el color de fondo
  pub fn clear(&mut self) {
      for pixel in &mut self.buffer {
          *pixel = self.background_color;
      }
  }

  // Dibuja un punto en el framebuffer
  pub fn point(&mut self, x: usize, y: usize) {
      if x < self.width && y < self.height {
          self.buffer[y * self.width + x] = self.current_color;
      }
  }
}
