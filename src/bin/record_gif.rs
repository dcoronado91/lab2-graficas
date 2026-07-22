//! Runs the simulation and saves it as an animated GIF, so a demo can be
//! produced without any external screen-recording tool.
//!
//! Usage: `cargo run --bin record_gif [output_path] [frame_count]`
//! Defaults to `docs/demo.gif` and 150 frames (~12s at 12 fps).

use game_of_life::framebuffer::Framebuffer;
use game_of_life::game_of_life::step;
use game_of_life::patterns;
use raylib::prelude::*;
use std::env;
use std::fs::{self, File};
use std::path::Path;

const FB_WIDTH: i32 = 120;
const FB_HEIGHT: i32 = 120;
const SCALE: f32 = 6.0;
const TARGET_FPS: u32 = 12;

fn main() {
    let mut args = env::args().skip(1);
    let output_path = args.next().unwrap_or_else(|| "docs/demo.gif".to_string());
    let frame_count: usize = args.next().and_then(|s| s.parse().ok()).unwrap_or(150);

    if let Some(parent) = Path::new(&output_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).expect("failed to create output directory");
        }
    }

    let window_width = (FB_WIDTH as f32 * SCALE) as i32;
    let window_height = (FB_HEIGHT as f32 * SCALE) as i32;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life - grabando gif")
        .build();

    rl.set_target_fps(TARGET_FPS);

    let mut framebuffer = Framebuffer::new(FB_WIDTH, FB_HEIGHT, Color::BLACK);
    patterns::spawn_initial_pattern(&mut framebuffer);

    let mut texture = rl
        .load_texture_from_image(&thread, &framebuffer.color_buffer)
        .expect("failed to create texture from framebuffer");

    let gif_file = File::create(&output_path).expect("failed to create gif file");
    let mut encoder = gif::Encoder::new(gif_file, FB_WIDTH as u16, FB_HEIGHT as u16, &[])
        .expect("failed to create gif encoder");
    encoder
        .set_repeat(gif::Repeat::Infinite)
        .expect("failed to set gif repeat");

    // GIF delay is in hundredths of a second.
    let delay_centiseconds = (100 / TARGET_FPS) as u16;

    let mut frames_written = 0usize;

    while !rl.window_should_close() && frames_written < frame_count {
        step(&mut framebuffer);

        let mut pixels = framebuffer.color_buffer.get_image_data_u8(false);
        texture
            .update_texture(&pixels)
            .expect("failed to upload framebuffer to texture");

        let mut gif_frame =
            gif::Frame::from_rgba_speed(FB_WIDTH as u16, FB_HEIGHT as u16, &mut pixels, 10);
        gif_frame.delay = delay_centiseconds;
        encoder
            .write_frame(&gif_frame)
            .expect("failed to write gif frame");
        frames_written += 1;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(&texture, Vector2::new(0.0, 0.0), 0.0, SCALE, Color::WHITE);
        d.draw_fps(10, 10);
        d.draw_text(
            &format!("grabando gif: {}/{}", frames_written, frame_count),
            10,
            30,
            20,
            Color::LIME,
        );
    }

    drop(encoder);
    println!("Gif guardado en {} ({} frames)", output_path, frames_written);
}
