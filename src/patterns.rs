use crate::framebuffer::Framebuffer;
use crate::game_of_life::ALIVE;

/// Turns on every cell in `cells` (relative coordinates) offset by
/// (origin_x, origin_y). Every organism function below is just this
/// applied to a fixed list of relative coordinates for that pattern.
fn spawn(fb: &mut Framebuffer, origin_x: i32, origin_y: i32, cells: &[(i32, i32)]) {
    for &(dx, dy) in cells {
        fb.point(origin_x + dx, origin_y + dy, ALIVE);
    }
}

// ---- Still lifes ----

pub fn block(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(fb, x, y, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

pub fn beehive(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(
        fb,
        x,
        y,
        &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)],
    );
}

// ---- Oscillators ----

pub fn blinker(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(fb, x, y, &[(0, 0), (1, 0), (2, 0)]);
}

pub fn toad(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(
        fb,
        x,
        y,
        &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)],
    );
}

pub fn beacon(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(
        fb,
        x,
        y,
        &[(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)],
    );
}

/// Period-3 oscillator, 13x13 bounding box. Coordinates verified against
/// the canonical RLE (x = 13, y = 13, rule = B3/S23) from LifeWiki.
pub fn pulsar(fb: &mut Framebuffer, x: i32, y: i32) {
    const ARM: [i32; 6] = [2, 3, 4, 8, 9, 10];
    const BAR: [i32; 4] = [0, 5, 7, 12];
    const ARM_ROWS: [i32; 4] = [0, 5, 7, 12];
    const BAR_ROWS: [i32; 6] = [2, 3, 4, 8, 9, 10];

    for &row in ARM_ROWS.iter() {
        for &col in ARM.iter() {
            fb.point(x + col, y + row, ALIVE);
        }
    }
    for &row in BAR_ROWS.iter() {
        for &col in BAR.iter() {
            fb.point(x + col, y + row, ALIVE);
        }
    }
}

// ---- Spaceships ----

pub fn glider(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(fb, x, y, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

/// Lightweight spaceship. Coordinates verified against the canonical RLE
/// (x = 5, y = 4, rule = B3/S23 - catagolue xq4_6frc).
pub fn lwss(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(
        fb,
        x,
        y,
        &[
            (0, 0),
            (3, 0),
            (4, 1),
            (0, 2),
            (4, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (4, 3),
        ],
    );
}

// ---- Guns ----

/// The Gosper glider gun: 36 cells in a 36x9 box, period 30, the first
/// known pattern with unbounded growth. Coordinates decoded from the
/// canonical RLE (x = 36, y = 9, rule = B3/S23).
pub fn gosper_glider_gun(fb: &mut Framebuffer, x: i32, y: i32) {
    const CELLS: [(i32, i32); 36] = [
        (24, 0),
        (22, 1),
        (24, 1),
        (12, 2),
        (13, 2),
        (20, 2),
        (21, 2),
        (34, 2),
        (35, 2),
        (11, 3),
        (15, 3),
        (20, 3),
        (21, 3),
        (34, 3),
        (35, 3),
        (0, 4),
        (1, 4),
        (10, 4),
        (16, 4),
        (20, 4),
        (21, 4),
        (0, 5),
        (1, 5),
        (10, 5),
        (14, 5),
        (16, 5),
        (17, 5),
        (22, 5),
        (24, 5),
        (10, 6),
        (16, 6),
        (24, 6),
        (11, 7),
        (15, 7),
        (12, 8),
        (13, 8),
    ];
    spawn(fb, x, y, &CELLS);
}

// ---- Methuselahs ----

/// Acorn: only 7 cells, but takes 5206 generations to stabilize into
/// still lifes, oscillators and gliders - a small seed of long-term chaos.
pub fn acorn(fb: &mut Framebuffer, x: i32, y: i32) {
    spawn(
        fb,
        x,
        y,
        &[(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)],
    );
}

/// Populates the board with ~10 kinds of classic organisms spread out
/// across the whole framebuffer: still lifes, oscillators, spaceships,
/// a glider gun and a methuselah.
pub fn spawn_initial_pattern(fb: &mut Framebuffer) {
    // Guns: keep feeding gliders across the board for the whole run.
    gosper_glider_gun(fb, 2, 2);
    gosper_glider_gun(fb, 78, 100);

    // Pulsars: big, showy period-3 oscillators anchoring each quadrant.
    pulsar(fb, 50, 4);
    pulsar(fb, 95, 32);
    pulsar(fb, 8, 60);
    pulsar(fb, 60, 90);

    // Small oscillators scattered as filler.
    blinker(fb, 20, 30);
    toad(fb, 35, 28);
    beacon(fb, 100, 55);
    blinker(fb, 15, 90);
    toad(fb, 90, 15);
    beacon(fb, 30, 100);

    // Still lifes as quiet scenery.
    block(fb, 5, 45);
    beehive(fb, 30, 5);
    block(fb, 110, 20);
    beehive(fb, 5, 105);
    block(fb, 70, 65);
    beehive(fb, 105, 78);

    // Spaceships flying across open space.
    glider(fb, 8, 22);
    glider(fb, 100, 8);
    glider(fb, 8, 100);
    glider(fb, 45, 60);
    lwss(fb, 40, 15);
    lwss(fb, 15, 75);
    lwss(fb, 90, 45);

    // A methuselah left to slowly chew through empty space.
    acorn(fb, 55, 55);
}
