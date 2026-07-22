use raylib::prelude::Color;

use crate::framebuffer::Framebuffer;

pub const ALIVE: Color = Color::WHITE;
pub const DEAD: Color = Color::BLACK;

fn is_alive(color: Color) -> bool {
    color.r > 127
}

fn count_live_neighbors(fb: &Framebuffer, x: i32, y: i32) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if is_alive(fb.get_color(x + dx, y + dy)) {
                count += 1;
            }
        }
    }
    count
}

/// Advances the board by one generation, applying Conway's four rules to
/// every cell. The framebuffer is never cleared: every cell is explicitly
/// repainted alive or dead, which is what makes clearing unnecessary.
pub fn step(fb: &mut Framebuffer) {
    let cell_count = (fb.width * fb.height) as usize;
    let mut next_alive = vec![false; cell_count];

    for y in 0..fb.height {
        for x in 0..fb.width {
            let alive = is_alive(fb.get_color(x, y));
            let neighbors = count_live_neighbors(fb, x, y);

            let survives = match (alive, neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };

            next_alive[(y * fb.width + x) as usize] = survives;
        }
    }

    for y in 0..fb.height {
        for x in 0..fb.width {
            let alive = next_alive[(y * fb.width + x) as usize];
            fb.point(x, y, if alive { ALIVE } else { DEAD });
        }
    }
}
