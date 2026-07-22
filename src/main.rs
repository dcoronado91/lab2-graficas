mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use raylib::prelude::*;

const FB_WIDTH: i32 = 120;
const FB_HEIGHT: i32 = 120;
const SCALE: f32 = 6.0;
const TARGET_FPS: u32 = 12;

fn main() {
    let window_width = (FB_WIDTH as f32 * SCALE) as i32;
    let window_height = (FB_HEIGHT as f32 * SCALE) as i32;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut framebuffer = Framebuffer::new(FB_WIDTH, FB_HEIGHT, Color::BLACK);
    patterns::spawn_initial_pattern(&mut framebuffer);

    let mut texture = rl
        .load_texture_from_image(&thread, &framebuffer.color_buffer)
        .expect("failed to create texture from framebuffer");

    while !rl.window_should_close() {
        game_of_life::step(&mut framebuffer);

        texture
            .update_texture(&framebuffer.color_buffer.get_image_data_u8(false))
            .expect("failed to upload framebuffer to texture");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(&texture, Vector2::new(0.0, 0.0), 0.0, SCALE, Color::WHITE);
        d.draw_fps(10, 10);
    }
}
