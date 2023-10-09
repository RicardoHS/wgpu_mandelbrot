#[path = "../algcolor.rs"]
mod algcolor;
use clap::Parser;

use image::ImageBuffer;
use num::complex::Complex;
use rayon::prelude::*;

/// Render in PNG a portion of the mandelbrot set
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of max iterations to set pixel as diverged value
    #[arg(short, long, default_value = "500")]
    max_iter: u32,
    /// X (real) coordinate to plot in the center of the image
    #[arg(short, long, default_value = "-0.9170575")]
    x_coord: f64,
    /// Y (imaginary) coordinate to plot in the center of the image
    #[arg(short, long, default_value = "-0.277587")]
    y_coord: f64,
    /// Amount of zoom in the fractal
    #[arg(short, long, default_value = "35000.")]
    scale: f64,
    /// Number of pixels for Weight and Heigh
    #[arg(short, long, default_value = "1000")]
    img_size: u32,
}

fn mandelbrot_divergence(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z: Complex<f64> = Complex { re: 0., im: 0. };
    let mut n = 0;
    loop {
        z = z * z + c;
        n += 1;

        if z.norm() > 2. || n >= max_iter {
            break n;
        }
    }
}

fn main() {
    // takes 1.726 total seconds to complete on my M1
    let args = Args::parse();
    let min_x: f64 = args.x_coord + (-2. / args.scale);
    let max_x: f64 = args.x_coord + (2. / args.scale);
    let min_y: f64 = args.y_coord + (-2. / args.scale);
    let max_y: f64 = args.y_coord + (2. / args.scale);

    let c1 = algcolor::RGB::hex("E810DD");
    let c2 = algcolor::RGB::hex("33B242");
    let c3 = algcolor::RGB::hex("E81000");
    let c4 = algcolor::RGB::hex("1A246D");
    let palette = algcolor::new_linear_palette(c1, c2, c3, c4, args.max_iter);

    let mut image = ImageBuffer::new(args.img_size, args.img_size);

    let pixels: Vec<(u32, u32)> = (0..args.img_size)
        .flat_map(|i| (0..args.img_size).map(move |j| (i, j)))
        .collect();

    let mapped_pixels: Vec<[u8; 3]> = pixels
        .par_iter()
        .map(|&p| {
            let c = Complex {
                re: f64::from((p.0 as f64 / args.img_size as f64) * (max_x - min_x) + min_x),
                im: f64::from((p.1 as f64 / args.img_size as f64) * (max_y - min_y) + min_y),
            };
            let n = mandelbrot_divergence(c, args.max_iter);
            [
                palette[(n - 1) as usize].r,
                palette[(n - 1) as usize].g,
                palette[(n - 1) as usize].b,
            ]
        })
        .collect();

    for (pix, mpix) in std::iter::zip(pixels, mapped_pixels) {
        *image.get_pixel_mut(pix.0, pix.1) = image::Rgb(mpix);
    }

    //*image.get_pixel_mut(args.img_size / 2, args.img_size / 2) = image::Rgb([255, 0, 0]);
    image.save("output.png").unwrap();
}
