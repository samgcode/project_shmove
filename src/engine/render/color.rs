pub struct Color {
  red: f64,
  green: f64,
  blue: f64,
  hue: f64,
  saturation: f64,
  value: f64,
}

impl Color {
  pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
    let (hue, saturation, value) = rgb_to_hsv(r, g, b);

    Self {
      red: r,
      green: g,
      blue: b,
      hue,
      saturation,
      value,
    }
  }

  pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
    let (r, g, b) = hsv_to_rbg(h, s, v);

    Self {
      red: r,
      green: g,
      blue: b,
      hue: h,
      saturation: s,
      value: v,
    }
  }

  pub fn set_hue(&mut self, h: f64) {
    self.hue = h;
    while self.hue > 360.0 {
      self.hue -= 360.0;
    }
    let (r, g, b) = hsv_to_rbg(self.hue, self.value, self.saturation);
    self.red = r;
    self.green = g;
    self.blue = b;
  }

  pub fn to_wgpu(&self) -> wgpu::Color {
    wgpu::Color {
      r: self.red,
      g: self.green,
      b: self.blue,
      a: 1.0,
    }
  }
}

fn hsv_to_rbg(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
  let hue = h;

  let c = v * s;
  let mut a = hue / 60.0;
  while a > 2.0 {
    a -= 2.0;
  }

  let x = c * (1.0 - f64::abs(a - 1.0));
  let m = v - c;

  let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);

  if hue < 60.0 {
    (r, g, b) = (c, x, 0.0);
  } else if hue < 120.0 {
    (r, g, b) = (x, c, 0.0);
  } else if hue < 180.0 {
    (r, g, b) = (0.0, c, x);
  } else if hue < 240.0 {
    (r, g, b) = (0.0, x, c);
  } else if hue < 300.0 {
    (r, g, b) = (x, 0.0, c);
  } else if hue < 360.0 {
    (r, g, b) = (c, 0.0, x);
  }

  (r + m, g + m, b + m)
}

fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
  let mut hue = 0.0;
  let mut saturation = 0.0;

  let c_max = f64::max(f64::max(r, g), b);
  let c_min = f64::min(f64::min(r, g), b);
  let delta = c_max - c_min;

  let value = c_max;

  if delta == 0.0 {
    hue = 0.0;
  } else if r > g && r > b {
    hue = 60.0 * (g - b) / delta;
  } else if g > r && g > b {
    hue = 60.0 * ((b - r) / delta + 2.0);
  } else if b > g && b > r {
    hue = 60.0 * ((r - g) / delta + 4.0);
  }

  if c_max != 0.0 {
    saturation = delta / c_max;
  }
  (hue, saturation, value)
}
