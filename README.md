# wgpu_mandelbrot
Run mandelbrot set render using WebGPU, in web or locally.

# Usage

### Live explorer

Compile the project and run it locally with `make`.

For a wasm binary to execute it on a web browser, compile it with `make build-web` and open `index.html`.

### PNG generation

You can generate PNG images of portions of the mandelbrot set with a cli. It will output a file called `output.png`
with the render.

You can build a local PNG render cli with `make build-cli`. This will leave an executable 
`./target/release/multi_thread`. You can execute it with `-h` to see available options.

There is also available the same cli without multithreading (slower). You can compile it with 
`cargo r --release --bin single_thread` and check usage with `./target/release/single_thread -h`.
