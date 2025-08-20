use crate::COLORS;
use penrose::util::spawn_for_output_with_args;
use penrose_ui::{
    bar::widgets::{IntervalText, Text},
    core::TextStyle,
};
use std::time::Duration;

struct Volume {
    percentage: i32,
    is_mute: bool,
}

impl Volume {
    fn new() -> Self {
        let stats = spawn_for_output_with_args("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .unwrap_or_else(|_| "!".to_owned());

        let percentage = stats
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<f32>().ok()) // parse "1.23" as a float
            .map(|n| (n * 100.0).round() as i32) // scale to 123
            .unwrap_or(-1); // -1 means error

        let is_mute = stats.contains("MUTED");

        Self {
            percentage,
            is_mute,
        }
    }

    fn style(&self) -> TextStyle {
        let color = if self.is_mute || self.percentage > 100 {
            COLORS.red
        } else {
            COLORS.foreground
        };

        TextStyle {
            fg: color.into(),
            bg: Some(COLORS.background),
            padding: (1, 1),
        }
    }

    fn text(&self) -> String {
        match (self.is_mute, self.percentage > 100) {
            (true, _) => "X".to_string(),
            (false, true) => "~".to_string(),
            (false, false) => format!("{}%", self.percentage),
        }
    }
}

pub fn value() -> IntervalText {
    IntervalText::new(
        || Volume::new().style(),
        || Some(Volume::new().text()),
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
    Text::new("VOL".trim(), style, false, true)
}
