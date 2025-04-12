use penrose::util::spawn_for_output_with_args;
use penrose_ui::{bar::widgets::IntervalText, core::TextStyle};

use crate::{BAR_BACKGROUND, GRAY, GREEN, RED};
use std::time::Duration;

// fn values() -> GwmWidgetResult<(bool, i32)> {
//     let stats = spawn_for_output_with_args("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"]);

//     let percent = (stats
//         .split_whitespace()
//         .nth(1)
//         .unwrap()
//         .parse::<f32>()
//         .unwrap()
//         * 100.0)
//         .round() as i32;

//     let mute = stats.contains("MUTED");

//     Ok((mute, percent))
// }

// fn build() -> Option<String> {
//     let (mute, percent) = values().ok()?;
//     let icon = match (mute, percent) {
//         (true, _) => "󰸈",
//         (_, v) if v > 100 => "󱄡",
//         (_, v) if (60..=100).contains(&v) => "󰕾",
//         (_, v) if (30..=60).contains(&v) => "󰖀",
//         _ => "󰕿 ",
//     }
//     .to_string();

//     let res = format!(" {} ", icon);

//     Some(res)
// }

// pub fn widget(style: TextStyle) -> IntervalText {
//     IntervalText::new(style, build, Duration::from_secs(1))
// }
