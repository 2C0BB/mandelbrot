use macroquad::prelude::*;

const SCREEN_WIDTH: u32 = 500;
const SCREEN_HEIGHT: u32 = 500;

use std::ops::{Add, Sub, Mul, Div};
fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T 
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Div<T, Output=T>
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

struct ComplexNum<T> {
    a: T,
    b: T,
}

impl<T> ComplexNum<T>
where T: Mul<T, Output = T> +
Add<T, Output = T> +
Sub<T, Output = T> +
Copy
{
    fn squared(&self) -> Self {
        Self {
            a: self.a * self.a - self.b * self.b,
            b: (self.a * self.b) + (self.a * self.b)
        }
    }

    fn add(&self, other: &ComplexNum<T>) -> ComplexNum<T> {
        ComplexNum {
            a: self.a + other.a,
            b: self.b + other.b
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot Set".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        //fullscreen: true,
        ..Default::default()
    }
}

fn generate_pixels(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max: u32
) -> Texture2D {
    //let max = 110;
    let mut pixels: Vec<u8> = Vec::new();

    for py in 0..SCREEN_HEIGHT {
        for px in 0..SCREEN_WIDTH {

            let scaled_px: f64 = map_range((0.0, SCREEN_WIDTH as f64), (x_min, x_max), px as f64);
            let scaled_py: f64 = map_range((0.0, SCREEN_HEIGHT as f64), (y_min, y_max), py as f64);

            let c: ComplexNum<f64> = ComplexNum{a: scaled_px as f64, b: scaled_py as f64};
            let mut z: ComplexNum<f64> = ComplexNum{a: 0.0, b: 0.0};
            /*
            let c: ComplexNum<f64> = ComplexNum{a: -0.9, b: 0.27015};
            let mut z: ComplexNum<f64> = ComplexNum{a: scaled_px as f64, b: scaled_py as f64};
            */

            let mut n = 0;
            while n < max {
                z = z.squared().add(&c);

                if z.a * z.a + z.b * z.b > 4.0 {
                    break;
                }
                
                n += 1;
            }

            if n == max {
                n = 0;
            }

            let brightness = map_range((0.0, max as f64), (0.0, 1.0), n as f64) as f32;
            let colour = macroquad::color::hsl_to_rgb(brightness, 1.0, 0.5);
            pixels.push(map_range((0.0, 1.0), (0.0, 255.0), colour.r) as u8);
            pixels.push(map_range((0.0, 1.0), (0.0, 255.0), colour.g) as u8);
            pixels.push(map_range((0.0, 1.0), (0.0, 255.0), colour.b) as u8);
            pixels.push(map_range((0.0, 1.0), (0.0, 255.0), colour.a) as u8);

            /*
            let brightness = map_range((0.0, max as f64), (0.0, 255.0), n as f64) as u8;
            pixels.push((brightness << 3) as u8);
            pixels.push((brightness << 5) as u8);
            pixels.push((brightness * 4) as u8);
            pixels.push(255);
            */
        }
    }

    Texture2D::from_rgba8(SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, &pixels)
}

enum ZoomMode {
    X, Y
}

enum ZoomSide {
    Min, Max
}

const ZOOM_VEL: u32 = 3;

#[macroquad::main(window_conf)]
async fn main() {

    let mut x_min: f64 = -2.0;
    let mut x_max: f64 = 0.47;

    let mut y_min: f64 = -1.12;
    let mut y_max: f64 = 1.12;

    let mut screen_min_x: u32 = 0;
    let mut screen_max_x: u32 = SCREEN_WIDTH / 8;

    let mut screen_min_y: u32 = 0;
    let mut screen_max_y: u32 = SCREEN_HEIGHT / 8;

    let mut current_mode: ZoomMode = ZoomMode::X;
    let mut current_side: ZoomSide = ZoomSide::Min;

    let mut max: u32 = 10;

    let mut text: Texture2D = generate_pixels(
        x_min,
        x_max,
        y_min,
        y_max,
        max,
    );

    loop {

        if is_key_down(KeyCode::X) {
            current_mode = ZoomMode::X;
        }
        if is_key_down(KeyCode::Y) {
            current_mode = ZoomMode::Y;
        }

        if is_key_down(KeyCode::L) {
            current_side = ZoomSide::Min;
        }
        if is_key_down(KeyCode::R) {
            current_side = ZoomSide::Max;
        }
/*
        if is_key_down(KeyCode::Left) {
            match current_mode {
                ZoomMode::X => {
                    match current_side {
                        ZoomSide::Min => {
                            screen_min_x -= ZOOM_VEL;
                        },
                        ZoomSide::Max => {
                            screen_max_x -= ZOOM_VEL;
                        },
                    }
                },
                ZoomMode::Y => {
                    match current_side {
                        ZoomSide::Min => {
                            screen_min_y -= ZOOM_VEL;
                        },
                        ZoomSide::Max => {
                            screen_max_y -= ZOOM_VEL;
                        },
                    }
                },
            }
        }

        if is_key_down(KeyCode::Right) {
            match current_mode {
                ZoomMode::X => {
                    match current_side {
                        ZoomSide::Min => {
                            screen_min_x += ZOOM_VEL;
                        },
                        ZoomSide::Max => {
                            screen_max_x += ZOOM_VEL;
                        },
                    }
                },
                ZoomMode::Y => {
                    match current_side {
                        ZoomSide::Min => {
                            screen_min_y += ZOOM_VEL;
                        },
                        ZoomSide::Max => {
                            screen_max_y += ZOOM_VEL;
                        },
                    }
                },
            }
        }
*/

        if is_key_down(KeyCode::Left) {
            if screen_min_x >= ZOOM_VEL {
                screen_min_x -= ZOOM_VEL;
                screen_max_x -= ZOOM_VEL;
            }
        }
        if is_key_down(KeyCode::Right) {
            if screen_max_x + ZOOM_VEL < SCREEN_WIDTH - 1 {
                screen_min_x += ZOOM_VEL;
                screen_max_x += ZOOM_VEL;
            }
        }

        if is_key_down(KeyCode::Up) {
            if screen_min_y >= ZOOM_VEL {
                screen_min_y -= ZOOM_VEL;
                screen_max_y -= ZOOM_VEL;
            }
        }
        if is_key_down(KeyCode::Down) {
            if screen_max_y + ZOOM_VEL < SCREEN_HEIGHT - 1 {
                screen_min_y += ZOOM_VEL;
                screen_max_y += ZOOM_VEL;
            }
        }

        if is_key_pressed(KeyCode::Space) {
            let new_x_min = map_range((0.0, SCREEN_WIDTH as f64), (x_min, x_max), screen_min_x as f64);
            let new_x_max = map_range((0.0, SCREEN_WIDTH as f64), (x_min, x_max), screen_max_x as f64);

            let new_y_min = map_range((0.0, SCREEN_HEIGHT as f64), (y_min, y_max), screen_min_y as f64);
            let new_y_max = map_range((0.0, SCREEN_HEIGHT as f64), (y_min, y_max), screen_max_y as f64);

            x_min = new_x_min;
            x_max = new_x_max;

            y_min = new_y_min;
            y_max = new_y_max;

            text = generate_pixels(
                x_min,
                x_max,
                y_min,
                y_max,
                max
            );

            /*
            screen_min_x = 0;
            screen_max_x = SCREEN_WIDTH;
            */
        }
        if is_key_pressed(KeyCode::Enter) {
           max += 1;
            text = generate_pixels(
                x_min,
                x_max,
                y_min,
                y_max,
                max,
            );
        }

        if is_key_pressed(KeyCode::S) {
            text.get_texture_data().export_png("out.png");
        }


        clear_background(BLACK);
        draw_texture(&text, 0., 0., WHITE);

        draw_rectangle(screen_min_x as f32, 0.0, 1.0, SCREEN_HEIGHT as f32, RED);
        draw_rectangle(screen_max_x as f32, 0.0, 1.0, SCREEN_HEIGHT as f32, RED);

        draw_rectangle(0.0, screen_min_y as f32, SCREEN_WIDTH as f32, 1.0, RED);
        draw_rectangle(0.0, screen_max_y as f32, SCREEN_WIDTH as f32, 1.0, RED);

        next_frame().await;
    }
}
