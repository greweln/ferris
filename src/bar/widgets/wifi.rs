use crate::{COLORS, utils::helpers::read_sys_file};
use penrose_ui::{
    bar::widgets::{IntervalText, Text},
    core::TextStyle,
};
use std::time::Duration;

fn signal() -> String {
    let raw = read_sys_file("/proc/net/wireless");

    for line in raw.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            return format!("{}%", parts[2].trim_end_matches('.'));
        }
    }

    "X".to_string() // No signal
}

fn style() -> TextStyle {
    let color = if signal() == "X" {
        COLORS.red
    } else {
        COLORS.gray_light
    };

    TextStyle {
        fg: color.into(),
        bg: Some(COLORS.background),
        padding: (1, 1),
    }
}
pub fn value() -> IntervalText {
    IntervalText::new(
        || style(),
        || Some(signal()),
        Duration::from_secs(1),
        false,
        true,
    )
}

pub fn text() -> Text {
    let style = TextStyle {
        fg: COLORS.gray_light,
        bg: Some(COLORS.background),
        padding: (6, 4),
    };
    Text::new("WiFI".trim(), style, false, true)
}
