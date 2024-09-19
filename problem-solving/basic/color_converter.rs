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
            Color::HEX(hex) => todo!(),
        }
    }
    fn to_rgb(&self) -> Self {
        let color = self.clone();

        match color {
            Color::RGB(r, g, b) => Color::RGB(r, g, b),
            Color::HSL(h, s, l) => todo!(),
            Color::HEX(hex) => todo!(),
        }
    }
    fn to_hex(&self) -> Self {
        let color = self.clone();

        match color {
            Color::RGB(r, g, b) => todo!(),
            Color::HSL(h, s, l) => todo!(),
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
    fn test_rgb_to_hsl() {
        let rgb = Color::RGB(255, 255, 255);
        assert_eq!(rgb.to_hsl(), Color::HSL(0, 0, 100));
    }
}
