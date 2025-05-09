use crate::{
    BAR_BACKGROUND, BAR_HEIGHT_PX, FONT, FONT_SIZE, FerrisResult, WS_BG, WS_FG, WS_HIGHLIGHT,
    bar::widgets::{battery, brackets, time_date, volume, wifi},
};
use penrose::x::XConn;
use penrose_ui::{
    StatusBar,
    bar::{Position, widgets::Workspaces},
    core::TextStyle,
};

pub fn config<X: XConn>() -> FerrisResult<StatusBar<X>> {
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
        vec![
            Box::new(workspaces),
            Box::new(brackets::open_greedy("[ BAT")),
            Box::new(battery::widget()),
            Box::new(brackets::close()),
            Box::new(brackets::open("[ WiFi")),
            Box::new(wifi::widget()),
            Box::new(brackets::close()),
            Box::new(brackets::open("[ VOL")),
            Box::new(volume::widget()),
            Box::new(brackets::close()),
            Box::new(time_date::widget()),
        ],
    )?;

    Ok(bar)
}
