trait InRange {
    fn in_range(self, begin: Self, end: Self) -> bool;
}

impl InRange for u16 {
    fn in_range(self, begin: u16, end: u16) -> bool {
        self >= begin && self < end
    }
}

#[derive(Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug)]
pub struct Hsl(pub u16, pub u16, pub u16);

fn rgb_to_hsl(v: Rgb) -> Hsl {
    let r = v.0 as f32 / 255.0;
    let g = v.1 as f32 / 255.0;
    let b = v.2 as f32 / 255.0;

    let data = vec![r, g, b];
    let c_min = data
        .clone()
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let c_max = data
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let delta: f32 = c_max - c_min;

    let mut h = 0.0;

    if c_max == r {
        h = 60.0 * (((g - b) / delta) % 6.0);
    } else if c_max == g {
        h = 60.0 * ((b - r) / delta + 2.0)
    } else if c_max == b {
        h = 60.0 * ((r - g) / delta + 4.0)
    }

    if h < 0.0 {
        h = h + 360.0;
    }

    let l = (c_max + c_min) / 2.0;

    let s = if delta != 0.0 {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    } else {
        0.0
    };

    Hsl(
        h.round() as u16,
        (s * 1000.0).ceil() as u16 / 10,
        (l * 1000.0).ceil() as u16 / 10,
    )
}

fn hsl_to_rgb(v: Hsl) -> Rgb {
    let h = v.0;
    let s = v.1 as f32 / 100.0;
    let l = v.2 as f32 / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h as f32 / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    println!("{}, {}, {}", c, x, m);

    let (r, g, b) = match h {
        n if n.in_range(0, 60) => (c, x, 0.0),
        n if n.in_range(60, 120) => (x, c, 0.0),
        n if n.in_range(120, 180) => (0.0, c, x),
        n if n.in_range(180, 240) => (0.0, x, c),
        n if n.in_range(240, 300) => (x, 0.0, c),
        n if n.in_range(300, 360) => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    println!("{}, {}, {}", r, g, b);

    Rgb(
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

impl From<Rgb> for Hsl {
    fn from(v: Rgb) -> Self {
        rgb_to_hsl(v)
    }
}

impl Into<Rgb> for Hsl {
    fn into(self) -> Rgb {
        hsl_to_rgb(self)
    }
}
