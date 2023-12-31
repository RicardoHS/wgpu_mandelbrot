build-local:
	cargo r --release --bin wgpu_mandelbrot

build-web:
	wasm-pack build --target web
	
build-cli:
	cargo r --release --bin multi_thread
