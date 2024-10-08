/// To run/test the binary, pease use the following commands:
/// * cargo run --bin color_converter
/// * cargo test --bin color_converter
use std::cmp::{max, min};

/// The enum `Color` can store one of the following values:
/// * RGB
/// * HSL
/// * HEX
/// Once Initialized, we can convert one type to another.
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
                // we first need to find minimum and maximum values which will
                // later be used to calculate other values
                let _min = min(min(r, g), b) as f32;
                let _max = max(max(r, g), b) as f32;

                // convert r,g,b to f32 since subtraction and division will result in a floating point value
                let r = r as f32;
                let g = g as f32;
                let b = b as f32;

                // find lightness
                // rgb ranges from 0 to 255 which needs to be normalized to
                // 0-100 in our case.
                // some converters also might use saturation and lightness in
                // range 0-1
                let l = ((_min + _max) / (2.0 * 2.55)).round() as u8;

                // find saturation
                // if all colors are same, then it will be the shade of gray.
                // example: rgb(40,40,40) is a shade of gray. if it is a shade
                // of gray, the saturation will always
                let s = if _min == _max {
                    0
                } else {
                    if l <= 50 {
                        (((_max - _min) / (_max + _min)) * 100.0).round() as u8
                    } else {
                        (((_max - _min) / (2.0 - _max - _min)) * 100.0).round() as u8
                    }
                };

                // find hue
                let h = if r >= g && r >= b {
                    (g - b) / (_max - _min)
                } else if g >= b && g >= r {
                    2.0 + (b - r) / (_max - _min)
                } else {
                    4.0 + (r - g) / (_max - _min)
                };

                let h = (h * 60.0) as u16;

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
                // if saturation is 0, then the color is grayscale and the
                // hue is unnecessary
                if s == 0 {
                    let r = (l as f32 * 255.0 / 100.0) as u8;
                    // for grayscale colors, all r,g, and b values are same
                    return Color::RGB(r, r, r);
                }

                // for ease, we normalize h,s, and l to range 0 to 1
                let hue = h as f32 / 360.0;
                let s = s as f32 / 100.0;
                let l = l as f32 / 100.0;

                // to convert HSL to RGB, we need 2 arbitrary values, on which
                // we perform calculations
                // * if l >=0.5, tmp1 = (l + s) - (l * s)
                // * if l < 0.5 tmp1 = l * (1 + s)
                let tmp1 = (if l >= 0.5 {
                    l + s - l * s
                } else {
                    l * (1.0 + s)
                }) as f32;

                let tmp2 = 2.0 * l - tmp1;

                // we first find temporary values of red, green, and blue since
                // they might not be in scale of 0 to 1;
                // after that, we need to add or subtract values accordingly to
                // get them in the range of 0 to 1 using normalize method.;

                let normalize = |v: f32| {
                    if v > 1.0 {
                        v - 1.0
                    } else if v < 0.0 {
                        v + 1.0
                    } else {
                        v
                    }
                };

                let tmp_r = normalize(hue + 0.333);
                let tmp_g = normalize(hue);
                let tmp_b = normalize(hue - 0.333);

                // calculation for r
                let r = ((if 6.0 * tmp_r < 1.0 {
                    tmp2 + (tmp1 - tmp2) * 6.0 * tmp_r
                } else if tmp_r * 2.0 < 1.0 {
                    tmp1
                } else if 3.0 * tmp_r < 2.0 {
                    tmp2 + (tmp1 - tmp2) * (0.666 - tmp_r) * 6.0
                } else {
                    tmp2
                }) * 255.0) as u8;

                // calculation for g
                let g = ((if 6.0 * tmp_g < 1.0 {
                    tmp2 + (tmp1 - tmp2) * 6.0 * tmp_g
                } else if tmp_g * 2.0 < 1.0 {
                    tmp1
                } else if 3.0 * tmp_g < 2.0 {
                    tmp2 + (tmp1 - tmp2) * (0.666 - tmp_g) * 6.0
                } else {
                    tmp2
                }) * 255.0) as u8;

                // calculation for b
                let b = ((if 6.0 * tmp_b < 1.0 {
                    tmp2 + (tmp1 - tmp2) * 6.0 * tmp_b
                } else if tmp_b * 2.0 < 1.0 {
                    tmp1
                } else if 3.0 * tmp_b < 2.0 {
                    tmp2 + (tmp1 - tmp2) * (0.666 - tmp_b) * 6.0
                } else {
                    tmp2
                }) * 255.0) as u8;

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
        assert_eq!(Color::RGB(00, 102, 170).to_hsl(), Color::HSL(204, 100, 33));
    }

    #[test]
    fn hex_to_other() {
        let hex = Color::HEX("#FFFFFF".to_owned());
        assert_eq!(hex.to_rgb(), Color::RGB(255, 255, 255));
        assert_eq!(hex.to_hsl(), Color::HSL(0, 0, 100));
    }

    #[test]
    fn hsl_to_other() {
        let hsl = Color::HSL(0, 0, 100);
        assert_eq!(hsl.to_rgb(), Color::RGB(255, 255, 255));
        assert_eq!(hsl.to_hex(), Color::HEX("#FFFFFF".to_owned()));

        // if we convert the RGB to HSL and then back to RGB, then slightly different
        // result is found due to rounding and conversion of f32 to u8 value
        // so color different conversion softwares might return slightly different values
        let hsl = Color::HSL(204, 100, 33);
        assert_eq!(hsl.to_rgb(), Color::RGB(0, 100, 168));
        assert_eq!(hsl.to_hex(), Color::HEX("#0064A8".to_owned()));
    }
}
