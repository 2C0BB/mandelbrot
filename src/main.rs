use macroquad::prelude::*;

const SCREEN_WIDTH: u32 = 1000;
const SCREEN_HEIGHT: u32 = 1000;

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
        ..Default::default()
    }
}

fn generate_pixels() -> Vec<Vec<Color>> {
    let mut out: Vec<Vec<Color>> = Vec::new();

    for py in 0..SCREEN_HEIGHT {
        let mut current_row: Vec<Color> = Vec::new();

        for px in 0..SCREEN_WIDTH {

            let scaled_px: f64 = map_range((0.0, SCREEN_WIDTH as f64), (-2.0, 0.47), px as f64);
            let scaled_py: f64 = map_range((0.0, SCREEN_HEIGHT as f64), (-1.12, 1.12), py as f64);

            let c: ComplexNum<f64> = ComplexNum{a: scaled_px as f64, b: scaled_py as f64};
            let mut z: ComplexNum<f64> = ComplexNum{a: 0.0, b: 0.0};

            let mut n = 0;
            while n < 100 {
                z = z.squared().add(&c);

                if z.a * z.a + z.b * z.b > 4.0 {
                    break;
                }
                
                n += 1;
            }

            let brightness = map_range((0.0, 100.0), (0.0, 1.0), n as f32);
            let pixel_colour: Color = Color { r: brightness, g: brightness, b: brightness, a: 1.0 };
            current_row.push(pixel_colour);
        }

        out.push(current_row);
    }

    out
}

#[macroquad::main(window_conf)]
async fn main() {

    let pixels = generate_pixels();
    let mut img = Image::gen_image_color(SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, WHITE);

    for (y, row) in pixels.iter().enumerate() {
        for (x, colour) in row.iter().enumerate() {
            img.set_pixel(x as u32, y as u32, *colour);
        }
    }
    img.export_png("out.png");
    let text = Texture2D::from_image(&img);


    loop {
        clear_background(BLACK);
        draw_texture(&text, 0., 0., WHITE);
        next_frame().await;
    }
}
