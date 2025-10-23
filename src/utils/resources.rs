use penrose::Color;
use std::collections::HashMap;

// Read and use the colors from Xresouces

pub struct Xresources {
    pub foreground: Color,
    pub background: Color,
    pub black: Color,
    pub gray: Color,
    pub red: Color,
    pub red_light: Color,
    pub green: Color,
    pub green_light: Color,
    pub yellow: Color,
    pub yellow_light: Color,
    pub blue: Color,
    pub blue_light: Color,
    pub magenta: Color,
    pub magenta_light: Color,
    pub cyan: Color,
    pub cyan_light: Color,
    pub gray_light: Color,
    pub white: Color,
}

impl Xresources {
    pub fn colors() -> Self {
        let output = std::process::Command::new("xrdb")
            .arg("-query")
            .output()
            .expect("Cannot run xrdb command");
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut colors: HashMap<String, Color> = stdout
            .lines()
            .filter_map(|line| {
                line.split_once(':').and_then(|(k, v)| {
                    let key = k.trim().strip_prefix("*.")?.to_string();
                    let val = v.trim();
                    hex_to_color(val).map(|color| (key, color))
                })
            })
            .collect();

        Self {
            foreground: get_color(&mut colors, "foreground"),
            background: get_color(&mut colors, "background"),
            black: get_color(&mut colors, "color0"),
            gray: get_color(&mut colors, "color8"),
            red: get_color(&mut colors, "color1"),
            red_light: get_color(&mut colors, "color9"),
            green: get_color(&mut colors, "color2"),
            green_light: get_color(&mut colors, "color10"),
            yellow: get_color(&mut colors, "color3"),
            yellow_light: get_color(&mut colors, "color11"),
            blue: get_color(&mut colors, "color4"),
            blue_light: get_color(&mut colors, "color12"),
            magenta: get_color(&mut colors, "color5"),
            magenta_light: get_color(&mut colors, "color13"),
            cyan: get_color(&mut colors, "color6"),
            cyan_light: get_color(&mut colors, "color14"),
            gray_light: get_color(&mut colors, "color7"),
            white: get_color(&mut colors, "color15"),
        }
    }
}

fn get_color(colors: &mut HashMap<String, Color>, key: &str) -> Color {
    colors
        .remove(key)
        .unwrap_or_else(|| Color::new_from_hex(0x000000FF))
}

// Convert a hex string like "#RRGGBB" into a penrose::Color, which is internally represented as 0xRRGGBBAA (Red, Green, Blue, Alpha).
// Removes the # prefix.
// Parses the remaining RRGGBB into a u32.
// Shifts it left by 8 bits to make space for an 8-bit alpha.
// Adds 0xFF to the lower 8 bits for full opacity (alpha = 255).
// Returns a penrose::Color created with that 0xRRGGBBAA value.
fn hex_to_color(s: &str) -> Option<Color> {
    let hex = s.strip_prefix('#')?;
    if hex.len() == 6 {
        let rgb = u32::from_str_radix(hex, 16).ok()?;
        let rgba = (rgb << 8) + 0xFF; // Add alpha = 0xFF
        Some(Color::new_from_hex(rgba))
    } else {
        None
    }
}
