use crate::{
    BAR_BACKGROUND, BAR_HEIGHT_PX, FONT, FONT_SIZE, FerrisResult, WS_BG, WS_FG, WS_HIGHLIGHT,
    widgets::time_date,
};
use penrose::x::XConn;
use penrose_ui::{
    StatusBar,
    bar::{Position, widgets::Workspaces},
    core::TextStyle,
};

pub fn bar<X: XConn>() -> FerrisResult<StatusBar<X>> {
    let workspace_style = TextStyle {
        fg: WS_FG.into(),
        bg: Some(WS_BG.into()),
        padding: (0, 0),
    };

    let workspaces = Workspaces::new(workspace_style, WS_HIGHLIGHT, WS_BG);

    let bar = StatusBar::try_new(
        Position::Top,
        BAR_HEIGHT_PX,
        BAR_BACKGROUND,
        FONT,
        FONT_SIZE,
        vec![Box::new(workspaces), Box::new(time_date::time_date())],
    )?;

    Ok(bar)
}
