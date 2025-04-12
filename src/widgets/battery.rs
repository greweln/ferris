use penrose::util::spawn_for_output_with_args;
use penrose_ui::{bar::widgets::IntervalText, core::TextStyle};

use crate::{BAR_BACKGROUND, GREEN, RED};
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum BatteryStatus {
    Charging,
    Discharging,
    Unknown,
    NotCharging,
    Full,
}

struct Battery {
    status: BatteryStatus,
    percentage: u32,
}

impl Battery {
    fn new() -> Self {
        let status_raw =
            spawn_for_output_with_args("cat", &["/sys/class/power_supply/BAT0/status"])
                .unwrap_or_else(|_err| "?".to_owned());

        let percentage =
            spawn_for_output_with_args("cat", &["/sys/class/power_supply/BAT0/capacity"])
                .unwrap_or_else(|_err| "0".to_owned())
                .parse::<u32>()
                .unwrap_or(0);

        let status = match status_raw.trim() {
            "Discharging" => BatteryStatus::Discharging,
            "Charging" => BatteryStatus::Charging,
            "Not charging" => BatteryStatus::NotCharging,
            "Full" => BatteryStatus::Full,
            _ => BatteryStatus::Unknown,
        };

        Self { status, percentage }
    }

    fn style(&self) -> TextStyle {
        if self.percentage < 20 && self.status != BatteryStatus::Charging {
            TextStyle {
                fg: RED.into(),
                bg: Some(BAR_BACKGROUND.into()),
                padding: (6, 4),
            }
        } else {
            TextStyle {
                fg: GREEN.into(),
                bg: Some(BAR_BACKGROUND.into()),
                padding: (6, 4),
            }
        }
    }

    fn icon(&self) -> &'static str {
        match self.status {
            BatteryStatus::Discharging => "↓",
            BatteryStatus::Charging => "↑",
            BatteryStatus::NotCharging | BatteryStatus::Full => "",
            BatteryStatus::Unknown => "?",
        }
    }

    fn text(&self) -> String {
        format!("{}{}%", self.icon(), self.percentage)
    }
}

pub fn battery() -> IntervalText {
    IntervalText::new(
        Battery::new().style(),
        || Some(Battery::new().text()),
        Duration::from_secs(1),
        false,
        true,
    )
}
