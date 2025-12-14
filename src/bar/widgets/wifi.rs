use crate::COLORS;
use penrose::util::spawn_for_output_with_args;
use penrose_ui::{
    bar::widgets::{IntervalText, Text},
    core::TextStyle,
};
use std::time::Duration;

fn signal() -> String {
    let output = spawn_for_output_with_args(
        "nmcli",
        &["-t", "-f", "IN-USE,SIGNAL", "dev", "wifi", "list"],
    );

    if let Ok(output) = output {
        for line in output.lines() {
            if line.starts_with("*:") {
                let signal = line.trim_start_matches("*:");
                return format!("{}%", signal);
            }
        }
    }

    "X".to_string()
}

fn style() -> TextStyle {
    let color = if signal() == "X" {
        COLORS.red
    } else {
        COLORS.white
    };

    TextStyle {
        fg: color.into(),
        bg: Some(COLORS.black),
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
        fg: COLORS.white,
        bg: Some(COLORS.black),
        padding: (6, 4),
    };
    Text::new("WiFI".trim(), style, false, true)
}
