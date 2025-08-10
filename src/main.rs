use std::collections::HashMap;

use ferris::{
    BAR_HEIGHT_PX, COLORS, FerrisResult, INNER_PX, MAX_MAIN, OUTER_PX, RATIO_CENTER, RATIO_STEP,
    STARTUP_SCRIPT, TERMINAL, bar, keys,
};
use penrose::{
    builtin::{hooks::SpacingHook, layout::CenteredMain},
    core::{Config, WindowManager, bindings::parse_keybindings_with_xmodmap},
    extensions::hooks::{
        NamedScratchPad, SpawnOnStartup, add_ewmh_hooks, add_named_scratchpads,
        manage::FloatingCentered,
    },
    stack,
    x::query::AppName,
    x11rb::RustConn,
};
use tracing_subscriber::{self, prelude::*};

fn logs() {
    tracing_subscriber::fmt()
        .with_env_filter("info") //trace
        .finish()
        .init();
}

fn config() -> Config<RustConn> {
    let layout = stack! {CenteredMain::vertical(MAX_MAIN, RATIO_CENTER, RATIO_STEP)};

    // Hooks
    let layout_hook = Box::new(SpacingHook {
        inner_px: INNER_PX,
        outer_px: OUTER_PX,
        top_px: BAR_HEIGHT_PX,
        bottom_px: 0,
    });

    let startup_hook = Box::new(vec![SpawnOnStartup::boxed(STARTUP_SCRIPT)]);

    Config {
        normal_border: COLORS.black,
        focused_border: COLORS.foreground,
        border_width: 1,
        focus_follow_mouse: true,
        default_layouts: layout,
        startup_hook: Some(startup_hook),
        layout_hook: Some(layout_hook),
        tags: ["1", "2", "3", "4", "5", "6"]
            .iter()
            .map(|w| w.to_string())
            .collect(),
        ..Config::default()
    }
}

fn main() -> FerrisResult<()> {
    logs();
    let conn = RustConn::new()?;

    // Scratchpads
    let (spt, terminal) = NamedScratchPad::new(
        "terminal",
        format!("{} -n SpTerm", TERMINAL),
        AppName("SpTerm"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );

    let (spg, gpass) = NamedScratchPad::new(
        "gpass",
        format!(
            "timeout  2m {} -n SpGpass -e gpass /home/me/gpass.json",
            TERMINAL
        ),
        AppName("SpGpass"),
        FloatingCentered::new(0.8, 0.8),
        true,
    );

    // Bar
    let bar = bar::config::config().unwrap();
    let key_bindings = parse_keybindings_with_xmodmap(keys::key_bindings(gpass, terminal))?;
    let config = add_ewmh_hooks(config());
    let wm = WindowManager::new(config, key_bindings, HashMap::new(), conn)?;
    let wm = bar.add_to(wm);

    // Add scratchpads, bar to the WM
    let wm = add_named_scratchpads(wm, vec![spt, spg]);

    // Run the Window Manager
    let ferris = wm.run()?;

    Ok(ferris)
}
