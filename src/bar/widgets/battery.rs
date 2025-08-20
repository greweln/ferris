use crate::{COLORS, utils::helpers::read_sys_file};
use penrose_ui::{
    bar::widgets::{IntervalText, Text},
    core::TextStyle,
};

use std::time::Duration;

struct Battery {
    percentage: u32,
    is_plugged: bool,
}

impl Battery {
    fn new() -> Self {
        let percentage = read_sys_file("/sys/class/power_supply/BAT0/capacity")
            .parse::<u32>()
            .unwrap_or(0);

        let is_plugged = read_sys_file("/sys/class/power_supply/AC/online");

        let is_plugged = match is_plugged.as_str() {
            "1" => true,
            _ => false,
        };

        Self {
            percentage,
            is_plugged,
        }
    }

    fn style(&self) -> TextStyle {
        let color = if self.is_plugged {
            COLORS.green
        } else {
            if self.percentage < 20 {
                COLORS.red
            } else {
                COLORS.foreground
            }
        };

        TextStyle {
            fg: color.into(),
            bg: Some(COLORS.background),
            padding: (1, 1),
        }
    }

    fn text(&self) -> String {
        format!("{}%", self.percentage)
    }
}

pub fn value() -> IntervalText {
    IntervalText::new(
        || Battery::new().style(),
        || Some(Battery::new().text()),
        Duration::from_secs(1),
        false,
        true,
    )
}

pub fn text() -> Text {
    let style = TextStyle {
        fg: COLORS.foreground,
        bg: Some(COLORS.background),
        padding: (6, 4),
    };
    Text::new("BAT".trim(), style, false, true)
}
