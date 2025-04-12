use crate::{BAR_BACKGROUND, GRAY};
use penrose_ui::{TextStyle, bar::widgets::Text};

pub fn open() -> Text {
    let style = TextStyle {
        fg: GRAY.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };
    Text::new("[".trim(), style, true, true)
}

pub fn close() -> Text {
    let style = TextStyle {
        fg: GRAY.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };
    Text::new("]".trim(), style, false, true)
}

pub fn battery_open() -> Text {
    let style = TextStyle {
        fg: GRAY.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };
    Text::new("[ BAT".trim(), style, true, true)
}
