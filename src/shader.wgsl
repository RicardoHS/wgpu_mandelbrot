// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct FractalInfo {
    pos_x: f32,
    pos_y: f32,
    scale: f32,
    img_size_w: f32,
};
@group(0) @binding(0) 
var<uniform> fractalInfo: FractalInfo;

struct ScreenInfo {
    img_size_w: f32,
    img_size_h: f32,
    pad: f32,
    pad2: f32,
};
@group(1) @binding(0) 
var<uniform> screenInfo: ScreenInfo;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
fn complex_mul(c1: vec2<f32>, c2: vec2<f32>) -> vec2<f32>{
    return vec2<f32>(c1[0]*c2[0]-c1[1]*c2[1], c1[0]*c2[1] + c1[1]*c2[0]);
}

fn mandelbrot_divergence(c: vec2<f32>) -> f32 {
    let max_iter = 500.0;
    var z = vec2<f32>(0.0,0.0);
    var n = 0.0;
    loop {
        z = complex_mul(z, z) + c;
        n += 1.0;

        if length(z) >= 2.0 || n >= max_iter {
            break;
        }
    }
    return n/(max_iter);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let scale = fractalInfo.scale;
    let min_x = fractalInfo.pos_x + (-2. / scale);
    let max_x = fractalInfo.pos_x + (2. / scale);
    let min_y = fractalInfo.pos_y + (-2. / scale);
    let max_y = fractalInfo.pos_y + (2. / scale);

    let min_screen_size = min(screenInfo.img_size_w, screenInfo.img_size_h);

    let x = (in.clip_position.x / min_screen_size) * (max_x - min_x) + min_x;
    let y = (in.clip_position.y / min_screen_size) * (max_y - min_y) + min_y;

    let n = mandelbrot_divergence(vec2<f32>(x,y));

    return vec4<f32>(0.0, n, 0.0, 1.0);
}
