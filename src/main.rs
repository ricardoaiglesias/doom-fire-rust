extern crate sdl2;

use rand::prelude::*;

use sdl2::pixels::Color;

const FIRE_WIDTH: usize = 100;
const FIRE_HEIGHT: usize = 100;
const NUM_FIRE_COLORS: usize = 37;

#[allow(dead_code)]
enum AnimationType {
    Uniform,
    VerticalRand,
    TwoAxesRand, // Vertical and horizontal randomness.
}

// Maps from int -> Color. Number 36 will be "HOT" while 0 will be "No Fire".
// By decreasing the values, we get a gradual "cooling"animation.
//type PixelArray = [[usize; FIRE_WIDTH]; FIRE_HEIGHT];
type PixelArray = Vec<Vec<usize>>;
type ByteArray = [u8; FIRE_WIDTH * FIRE_HEIGHT * 3];
type FireColorArray = [Color; NUM_FIRE_COLORS];

const NUM_FRAMES: usize = 500;
fn main() {
    // Array that specifies which color goes in which pixel.
    let mut colors: PixelArray = vec![vec![0; FIRE_WIDTH]; FIRE_HEIGHT];
    let mut byte_pixel_array: ByteArray = [0; FIRE_WIDTH * FIRE_HEIGHT * 3];
    setup_pixel_array(&mut colors);

    // Create colors for fire.
    let fire_array = setup_fire_array();

    let mut canvas = setup_sdl();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(
            Some(sdl2::pixels::PixelFormatEnum::RGB24),
            FIRE_WIDTH as u32,
            FIRE_HEIGHT as u32,
        )
        .unwrap();

    for _ in 0..NUM_FRAMES {
        canvas.clear();
        fire_frame(&mut colors, AnimationType::TwoAxesRand);
        colors_to_bytes(&colors, &fire_array, &mut byte_pixel_array);

        texture
            .update(None, &byte_pixel_array, FIRE_WIDTH * 3)
            .unwrap();
        canvas.copy(&texture, None, None);
        canvas.present();
    }
}

// Runs ONE FRAME of the animation.
fn fire_frame(pixels: &mut PixelArray, animation: AnimationType) {
    let mut rng = rand::thread_rng();
    for row in 1..FIRE_HEIGHT {
        for col in 0..FIRE_WIDTH {
            match &animation {
                AnimationType::Uniform => spread_fire_uniform(pixels, row, col),
                AnimationType::VerticalRand => {
                    spread_fire_vertical_rand(pixels, &mut rng, row, col)
                }
                AnimationType::TwoAxesRand => spread_fire_two_axes(pixels, &mut rng, row, col),
            };
        }
    }
}

fn setup_sdl() -> sdl2::render::Canvas<sdl2::video::Window> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("DOOM FIRE", FIRE_HEIGHT as u32, FIRE_WIDTH as u32)
        .position_centered()
        .build()
        .unwrap();
    window.into_canvas().build().unwrap()
}

fn setup_pixel_array(pixels: &mut PixelArray) {
    // Initialize the bottom row to be the largest fire value:
    for col in 0..FIRE_WIDTH {
        pixels[FIRE_HEIGHT - 1][col] = NUM_FIRE_COLORS - 1;
    }
}

fn setup_fire_array() -> FireColorArray {
    let rgbs = [
        0x07, 0x07, 0x07, 0x1F, 0x07, 0x07, 0x2F, 0x0F, 0x07, 0x47, 0x0F, 0x07, 0x57, 0x17, 0x07,
        0x67, 0x1F, 0x07, 0x77, 0x1F, 0x07, 0x8F, 0x27, 0x07, 0x9F, 0x2F, 0x07, 0xAF, 0x3F, 0x07,
        0xBF, 0x47, 0x07, 0xC7, 0x47, 0x07, 0xDF, 0x4F, 0x07, 0xDF, 0x57, 0x07, 0xDF, 0x57, 0x07,
        0xD7, 0x5F, 0x07, 0xD7, 0x5F, 0x07, 0xD7, 0x67, 0x0F, 0xCF, 0x6F, 0x0F, 0xCF, 0x77, 0x0F,
        0xCF, 0x7F, 0x0F, 0xCF, 0x87, 0x17, 0xC7, 0x87, 0x17, 0xC7, 0x8F, 0x17, 0xC7, 0x97, 0x1F,
        0xBF, 0x9F, 0x1F, 0xBF, 0x9F, 0x1F, 0xBF, 0xA7, 0x27, 0xBF, 0xA7, 0x27, 0xBF, 0xAF, 0x2F,
        0xB7, 0xAF, 0x2F, 0xB7, 0xB7, 0x2F, 0xB7, 0xB7, 0x37, 0xCF, 0xCF, 0x6F, 0xDF, 0xDF, 0x9F,
        0xEF, 0xEF, 0xC7, 0xFF, 0xFF, 0xFF,
    ];

    let mut colors = [Color::RGB(0, 0, 0); NUM_FIRE_COLORS];

    for (index, color) in colors.iter_mut().enumerate() {
        let red = rgbs[3 * index];
        let green = rgbs[3 * index + 1];
        let blue = rgbs[3 * index + 2];

        *color = Color::RGB(red as u8, green as u8, blue as u8);
    }

    colors
}

fn draw_frame() {}

fn spread_fire_two_axes(
    pixels: &mut PixelArray,
    rng: &mut rand::rngs::ThreadRng,
    row: usize,
    col: usize,
) {
    // Random digit between 1 and 3
    let rand: usize = (rng.gen::<usize>() % 3) + 1;
    let prev_pixel = pixels[row][col];

    let dest_col = if col < (rand + 1) {
        col
    } else {
        col - rand + 1
    };
    if prev_pixel > 0 {
        let pixel_offset = if rand % 2 == 0 { 0 } else { 1 };
        pixels[row - 1][dest_col] = pixels[row][col] - pixel_offset;
    }
}

fn spread_fire_vertical_rand(
    pixels: &mut PixelArray,
    rng: &mut rand::rngs::ThreadRng,
    row: usize,
    col: usize,
) {
    // Get Random digit between 1 and 3
    let rand: usize = (rng.gen::<usize>() % 3) + 1;
    // Apply randomess.
    let prev_pixel = pixels[row][col];
    if prev_pixel > 0 {
        let offset = if rand % 2 == 0 { 0 } else { 1 };
        pixels[row - 1][col] = pixels[row][col] - offset;
    }
}

/*
 * Essentially, this scans across the entire array, and moves the last rows up
 * by one.
*/
fn spread_fire_uniform(pixels: &mut PixelArray, row: usize, col: usize) {
    // Have the current row be one less than the previous row. In effect, this
    // "cools down" the particles, since lower num
    let prev_pixel = pixels[row][col];
    if prev_pixel > 0 {
        // For example, row 35 will get one less than row 36's value.
        // Row 36 is the SOURCE of the fire, and propagates upward, one line per
        // frame.
        pixels[row - 1][col] = pixels[row][col] - 1;
    }
}

fn print_pixel_vals(pixels: PixelArray) {
    for (_, row) in pixels.iter().enumerate() {
        let mut prefix = "";
        for (_, pixel) in row.iter().enumerate() {
            print!("{}{}", prefix, pixel);
            prefix = ",\t";
        }
        println!()
    }
}

fn print_fire_array(fire: FireColorArray) {
    for (_, color) in fire.iter().enumerate() {
        println!("({}, {}, {})", color.r, color.g, color.b);
    }
}

fn colors_to_bytes(pixels: &PixelArray, fire_colors: &FireColorArray, bytes: &mut ByteArray) {
    for (byte, &pixel) in bytes.chunks_exact_mut(3).zip(pixels.into_iter().flatten()) {
        // Here, `byte` is an iterator over three bytes. Pixel is a single
        // index.
        let color = fire_colors[pixel];
        for (dest, src) in byte.iter_mut().zip(&[color.r, color.g, color.b]) {
            *dest = *src;
        }
    }
}
