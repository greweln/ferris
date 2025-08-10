use crate::COLORS;
use penrose_ui::{TextStyle, bar::widgets::Text};

pub fn open() -> Text {
    let style = TextStyle {
        fg: COLORS.black_bright,
        bg: Some(COLORS.black),
        padding: (6, 4),
    };
    Text::new("[".trim(), style, false, true)
}

// NOTE:the first widget's open bracket has to be greedy
// so that all widgets are pushed to the end of the bar
pub fn open_greedy() -> Text {
    let style = TextStyle {
        fg: COLORS.black_bright,
        bg: Some(COLORS.black),
        padding: (6, 4),
    };
    Text::new("[".trim(), style, true, true)
}

pub fn close() -> Text {
    let style = TextStyle {
        fg: COLORS.black_bright,
        bg: Some(COLORS.black),
        padding: (6, 4),
    };
    Text::new("]".trim(), style, false, true)
}
