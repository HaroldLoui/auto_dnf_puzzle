use std::{collections::HashMap, fmt::Display};
use image::Rgb;
use crate::config::GLOBAL_CONFIG;

lazy_static! {
    pub static ref COLOR_INDEX_MAP: HashMap<ColorIndex, (u32, u32)> = {
        let row_colors = GLOBAL_CONFIG.get_row_colors().unwrap();
        let col_colors = GLOBAL_CONFIG.get_col_colors().unwrap();
        let mut map = HashMap::new();
        for i in 0..row_colors.len() {
            for j in 0..col_colors.len() {
                map.insert(
                    ColorIndex::new(
                        Color::from_hex(&col_colors[j]),
                        Color::from_hex(&row_colors[i]),
                    ),
                    (i as u32, j as u32),
                );
            }
        }
        map
    };
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ColorIndex((Color, Color));

impl ColorIndex {
    pub fn new(c1: Color, c2: Color) -> Self {
        ColorIndex((c1, c2))
    }
}

/// 对`image::Rgb`的简单封装
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Color {
    rgb: Rgb<u8>,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            rgb: Rgb([255, 255, 255]),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb: {:?}, hex: {}", self.rgb, rgb_to_hex(self.rgb))
    }
}

impl Color {
    /// 16进制转为`Color`
    pub fn from_hex(hex: &str) -> Self {
        match hex_to_rgb(hex) {
            Ok(rgb) => Color { rgb },
            Err(_) => Color::default(),
        }
    }

    /// `rgb`转为`Color`
    pub fn from_rgb(rgb: Rgb<u8>) -> Self {
        Color { rgb }
    }
}

/// 16进制转为`rgb`
/// 
/// 返回类型：`Result<Rgb<u8>, &'static str>`
fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, &'static str> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Invalid hex color format");
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

    Ok(Rgb([r, g, b]))
}

/// `rgb`进制转为16
/// 
/// 返回类型：`String`
fn rgb_to_hex(rgb: Rgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}