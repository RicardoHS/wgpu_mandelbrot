use wgpu_mandelbrot::run;

fn main() {
    pollster::block_on(run());
}
