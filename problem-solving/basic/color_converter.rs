use std::cmp::{max, min};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    RGB(u8, u8, u8),
    HSL(u16, u8, u8),
    HEX(String),
}

impl Color {
    fn to_hsl(&self) -> Self {
        let color = self.clone();
        match color {
            Color::RGB(r, g, b) => {
                let _min = min(min(r, g), b);
                let _max = max(max(r, g), b);

                let l = ((_min as f32 + _max as f32) / (2f32 * 2.55)).round() as u8;
                let s = if _min == _max {
                    0
                } else {
                    if l <= 50 {
                        (((_max - _min) as f32 / (_max + _min) as f32) * 100f32).round() as u8
                    } else {
                        (((_max - _min) as f32 / (2 - _max - _min) as f32) * 100f32).round() as u8
                    }
                };
                let h = if r >= g && r >= b {
                    (g - b) as f32 / (_max - _min) as f32
                } else if g >= b && g >= r {
                    2f32 + (b - r) as f32 / (_max - _min) as f32
                } else {
                    4f32 + (r - g) as f32 / (_max - _min) as f32
                };

                let h = (h * 60f32) as u16;

                Color::HSL(h, s, l)
            }
            Color::HSL(h, s, l) => Color::HSL(h, s, l),
            Color::HEX(_) => color.to_rgb().to_hsl(),
        }
    }
    fn to_rgb(&self) -> Self {
        let color = self.clone();

        match color {
            Color::RGB(r, g, b) => Color::RGB(r, g, b),
            Color::HSL(h, s, l) => {
                // we need tmp1 and tmp2 for calculating rgb values
                let tmp1 = (if l >= 50 { l + s - l * s } else { l * (1 + s) }) as f32;
                let tmp2 = (2f32 * l as f32 - tmp1) as f32;

                let hue = h as f32 / 360f32;

                let tmp_r = hue + 0.333;
                let tmp_g = hue;
                let tmp_b = hue - 0.333;

                let tmp_r = if tmp_r > 1f32 {
                    tmp_r - 1f32
                } else if tmp_r < 0f32 {
                    tmp_r + 1f32
                } else {
                    tmp_r
                };

                let tmp_g = if tmp_g > 1f32 {
                    tmp_g - 1f32
                } else if tmp_g < 0f32 {
                    tmp_g + 1f32
                } else {
                    tmp_g
                };

                let tmp_b = if tmp_b > 1f32 {
                    tmp_b - 1f32
                } else if tmp_b < 0f32 {
                    tmp_b + 1f32
                } else {
                    tmp_b
                };

                // calculation for r
                let r = ((if 6f32 * tmp_r < 1f32 {
                    tmp2 + (tmp1 - tmp2) * 6f32 * tmp_r
                } else if tmp_r * 2f32 < 1f32 {
                    tmp1
                } else if 3f32 * tmp_r < 2f32 {
                    tmp2 + (tmp1 - tmp2) * (0.666 - tmp_r) * 6f32
                } else {
                    tmp2
                }) * 255f32) as u8;

                // calculation for g
                let g = ((if 6f32 * tmp_g < 1f32 {
                    tmp2 + (tmp1 - tmp2) * 6f32 * tmp_g
                } else if tmp_g * 2f32 < 1f32 {
                    tmp1
                } else if 3f32 * tmp_g < 2f32 {
                    tmp2 + (tmp1 - tmp2) * (0.666 - tmp_g) * 6f32
                } else {
                    tmp2
                }) * 255f32) as u8;

                // calculation for b
                let b = ((if 6f32 * tmp_b < 1f32 {
                    tmp2 + (tmp1 - tmp2) * 6f32 * tmp_b
                } else if tmp_b * 2f32 < 1f32 {
                    tmp1
                } else if 3f32 * tmp_b < 2f32 {
                    tmp2 + (tmp1 - tmp2) * (0.666 - tmp_b) * 6f32
                } else {
                    tmp2
                }) * 255f32) as u8;

                Color::RGB(r, g, b)
            }
            Color::HEX(hex) => {
                let (_, rgb) = hex.split_at(1);
                println!("{:?}", rgb);
                let rgb = u32::from_str_radix(rgb, 16).unwrap();
                Color::RGB(
                    (rgb / 0x10000 as u32) as u8, // 0x10000 = 0xFFFF + 0x1
                    ((rgb % 0x10000) / (0x100)) as u8,
                    (rgb % 0x100) as u8,
                )
            }
        }
    }
    fn to_hex(&self) -> Self {
        let color = self.clone();

        match color {
            Color::RGB(r, g, b) => {
                Color::HEX(format!("#{:02x}{:02x}{:02x}", r, g, b).to_uppercase())
            }
            Color::HSL(_, _, _) => color.to_rgb().to_hex(),
            Color::HEX(hex) => Color::HEX(hex),
        }
    }
}

fn main() {
    let red = Color::RGB(255, 0, 0);
    println!("red HEX: {:?}", red.to_hex());
    println!("red RGB: {:?}", red.to_rgb());
    println!("red HSL: {:?}", red.to_hsl());

    let green = Color::RGB(0, 255, 0);
    println!("green HEX: {:?}", green.to_hex());
    println!("green RGB: {:?}", green.to_rgb());
    println!("green HSL: {:?}", green.to_hsl());

    let blue = Color::RGB(0, 0, 255);
    println!("blue HEX: {:?}", blue.to_hex());
    println!("blue RGB: {:?}", blue.to_rgb());
    println!("blue HSL: {:?}", blue.to_hsl());
}

#[cfg(test)]
mod tests {
    use crate::Color;

    #[test]
    fn rgb_to_other() {
        let rgb = Color::RGB(255, 255, 255);
        assert_eq!(rgb.to_hsl(), Color::HSL(0, 0, 100));
        assert_eq!(rgb.to_hex(), Color::HEX("#FFFFFF".to_owned()));

        let rgb = Color::RGB(00, 102, 170);
        assert_eq!(rgb.to_hex(), Color::HEX("#0066AA".to_owned()));
    }

    #[test]
    fn hex_to_other() {
        let hex = Color::HEX("#FFFFFF".to_owned());
        assert_eq!(hex.to_rgb(), Color::RGB(255, 255, 255));
        assert_eq!(hex.to_hsl(), Color::HSL(0, 0, 100));
    }

    #[test]
    fn hsl_to_other() {
        let hex = Color::HSL(0, 0, 100);
        assert_eq!(hex.to_rgb(), Color::RGB(255, 255, 255));
        // assert_eq!(hex.to_hsl(), Color::HSL(0, 0, 100));
    }
}
