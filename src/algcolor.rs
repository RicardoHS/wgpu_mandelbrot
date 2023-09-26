#[derive(Debug, Copy, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn hex(s: &str) -> RGB {
        RGB {
            r: u8::from_str_radix(&s[0..2], 16).unwrap(),
            g: u8::from_str_radix(&s[2..4], 16).unwrap(),
            b: u8::from_str_radix(&s[4..6], 16).unwrap(),
        }
    }
}

fn get_v_delta(c1: RGB, c2: RGB, d: u32) -> [f64; 3] {
    let r_inc: f64 = (c2.r as f64 - c1.r as f64) / d as f64;
    let g_inc: f64 = (c2.g as f64 - c1.g as f64) / d as f64;
    let b_inc: f64 = (c2.b as f64 - c1.b as f64) / d as f64;
    [r_inc, g_inc, b_inc]
}

pub fn new_linear_palette(c1: RGB, c2: RGB, c3: RGB, c4: RGB, n: u32) -> Vec<RGB> {
    let colors = [c1, c2, c3, c4];
    let n_colors: u32 = 4;
    let mut palette: Vec<RGB> = vec![];
    // number of interpolation points between provided colors
    let m: u32 = (n - n_colors) / (n_colors - 1);
    // extra points to the last interpolation segment if previous computation is not exact
    let r: u32 = (n - n_colors) % (n_colors - 1);

    // compute interpolation of values m times for each channel
    for i_seg in 0..(n_colors - 2) {
        palette.push(colors[i_seg as usize]);
        let ci1 = colors[i_seg as usize];
        let ci2 = colors[(i_seg + 1) as usize];
        let v_delta: [f64; 3] = get_v_delta(ci1, ci2, m + 2);
        for i in 1..(m + 1) {
            palette.push(RGB {
                r: (ci1.r as f64 + v_delta[0] * (i as f64)) as u8,
                g: (ci1.g as f64 + v_delta[1] * (i as f64)) as u8,
                b: (ci1.b as f64 + v_delta[2] * (i as f64)) as u8,
            })
        }
    }

    // last segment with r
    palette.push(colors[2]);
    let v_delta: [f64; 3] = get_v_delta(colors[2], colors[3], m + r + 2);
    for i in 1..(m + r + 1) {
        palette.push(RGB {
            r: (colors[2].r as f64 + v_delta[0] * (i as f64)) as u8,
            g: (colors[2].g as f64 + v_delta[1] * (i as f64)) as u8,
            b: (colors[2].b as f64 + v_delta[2] * (i as f64)) as u8,
        })
    }
    palette.push(colors[3]);

    palette
}
