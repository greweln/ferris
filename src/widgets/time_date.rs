use crate::{BAR_BACKGROUND, WHITE_BRIGHT};
use penrose::util::spawn_for_output_with_args;
use penrose_ui::{bar::widgets::IntervalText, core::TextStyle};

use std::time::Duration;

fn text() -> Option<String> {
    let output = spawn_for_output_with_args("date", &["+%a-%d-%b %H:%M:%S"]).ok()?;
    Some(output)
}

fn style() -> TextStyle {
    TextStyle {
        fg: WHITE_BRIGHT.into(),
        bg: Some(BAR_BACKGROUND.into()),
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
