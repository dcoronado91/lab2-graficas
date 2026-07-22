use raylib::prelude::*;

/// A CPU-side pixel buffer, smaller than the actual window, that gets
/// upscaled onto a texture every frame. `point` and `get_color` are the
/// only two operations the rest of the program needs to read/write cells.
pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, color);
        }
    }

    /// Reads back the color of a cell. Out-of-bounds coordinates are treated
    /// as dead (background color) - this is what gives the board "dead
    /// borders" instead of a toroidal wrap-around.
    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.get_color(x, y)
        } else {
            self.background_color
        }
    }
}
