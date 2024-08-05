pub fn color_custom(t: f32) -> [u8; 3] {
    let r = (255.0 * t) as u8;
    let g = (255.0 * (1.0 - t)) as u8;
    let b = (255.0 * (0.5 + t * 0.5)) as u8;
    [r, g, b]
}

pub fn color_palette(count: usize) -> [u8; 3] {
    let palette = [
        [66, 30, 15],
        [25, 7, 26],
        [9, 1, 47],
        [4, 4, 73],
        [0, 7, 100],
        [12, 44, 138],
        [24, 82, 177],
        [57, 125, 209],
        [134, 181, 229],
        [211, 236, 248],
        [241, 233, 191],
        [248, 201, 95],
        [255, 170, 0],
        [204, 128, 0],
        [153, 87, 0],
        [106, 52, 3],
    ];

    palette[count % palette.len()]
}

pub fn color_logarithmic_mapping(count: usize, limit: usize) -> [u8; 3] {
    let log_count = (count as f32).ln();
    let log_limit = (limit as f32).ln();
    let t = log_count / log_limit;
    let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
    let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
    let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;

    [r, g, b]
}

pub fn color_cubic_polynomial(t: f32) -> [u8; 3] {
    let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
    let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
    let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;

    [r, g, b]
}

pub fn color_linear_gradient(t: f32) -> [u8; 3] {
    let r = (255.0 * t) as u8;
    let g = (255.0 * (1.0 - t)) as u8;
    let b = (255.0 * t * (1.0 - t)) as u8;

    [r, g, b]
}

pub fn hue_rotation(t: f32) -> [u8; 3] {
    let hue = 360.0 * t;
    let saturation = 1.0;
    let value = 1.0;

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match hue as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}
