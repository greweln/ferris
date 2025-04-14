use crate::{BAR_BACKGROUND, GREY};
use penrose_ui::{TextStyle, bar::widgets::Text};

pub fn open(name: &str) -> Text {
    let style = TextStyle {
        fg: GREY.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };
    Text::new(name, style, false, true)
}

// NOTE:the first widget's open bracket has to be greedy
// so that all widgets are pushed to the end of the bar
pub fn open_greedy(name: &str) -> Text {
    let style = TextStyle {
        fg: GREY.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };
    Text::new(name, style, true, true)
}

pub fn close() -> Text {
    let style = TextStyle {
        fg: GREY.into(),
        bg: Some(BAR_BACKGROUND.into()),
        padding: (6, 4),
    };
    Text::new("]".trim(), style, false, true)
}
