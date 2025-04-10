use crate::{BAR_BACKGROUND, WHITE_BRIGHT, util::run_command_for_output_with_args};
use penrose_ui::{bar::widgets::IntervalText, core::TextStyle};

use std::time::Duration;

pub fn time_date() -> IntervalText {
    let style = TextStyle {
        fg: WHITE_BRIGHT.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };

    let text = || -> Option<String> {
        let output =
            run_command_for_output_with_args("date".to_string(), &["+%a-%d-%b %H:%M:%S"]).ok()?;
        Some(output)
    };

    IntervalText::new(style, text, Duration::from_secs(1), true, true)
}
