use crate::{
    BAR_HEIGHT_PX, COLORS, FONT, FONT_SIZE, FerrisResult,
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
        fg: COLORS.black_bright,
        bg: Some(COLORS.background),
        padding: (0, 0),
    };

    let workspaces = Workspaces::new(
        workspace_style,
        COLORS.yellow, // hightlight
        COLORS.background,
    );

    let bar = StatusBar::try_new(
        Position::Top,
        BAR_HEIGHT_PX,
        COLORS.black,
        FONT,
        FONT_SIZE,
        vec![
            Box::new(workspaces),
            Box::new(brackets::open_greedy()),
            Box::new(battery::text()),
            Box::new(battery::value()),
            Box::new(brackets::close()),
            Box::new(brackets::open()),
            Box::new(wifi::text()),
            Box::new(wifi::value()),
            Box::new(brackets::close()),
            Box::new(brackets::open()),
            Box::new(volume::text()),
            Box::new(volume::value()),
            Box::new(brackets::close()),
            Box::new(time_date::widget()),
        ],
    )?;

    Ok(bar)
}
