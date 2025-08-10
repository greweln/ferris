use crate::COLORS;
use penrose::util::spawn_for_output_with_args;
use penrose_ui::{bar::widgets::IntervalText, core::TextStyle};

use std::time::Duration;

fn text() -> Option<String> {
    let output = spawn_for_output_with_args("date", &["+%a-%d-%b %H:%M:%S"]).ok()?;
    Some(output)
}

fn style() -> TextStyle {
    TextStyle {
        fg: COLORS.white_bright,
        bg: Some(COLORS.black),
        padding: (1, 1),
    }
}

pub fn widget() -> IntervalText {
    IntervalText::new(
        || -> TextStyle { style() },
        || text(),
        Duration::from_secs(1),
        false,
        true,
    )
}
