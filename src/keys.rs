use crate::{TERMINAL, menu};
use penrose::{
    builtin::{
        actions::{exit, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::KeyEventHandler,
    extensions::{actions::toggle_fullscreen, hooks::ToggleNamedScratchPad},
    x11rb::RustConn,
};
use std::collections::HashMap;

fn fullscreen() -> Box<dyn KeyEventHandler<RustConn>> {
    toggle_fullscreen()
}

pub fn key_bindings(
    gpass: ToggleNamedScratchPad,
    terminal: ToggleNamedScratchPad,
) -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let bindings = vec![
        ("M-Return", spawn(TERMINAL)),
        ("M-S-space", modify_with(|cs| cs.swap_focus_and_head())),
        ("M-space", modify_with(|cs| cs.focus_up())),
        ("M-q", modify_with(|cs| cs.kill_focused())),
        ("M-k", modify_with(|cs| cs.focus_next_workspace())),
        ("M-j", modify_with(|cs| cs.focus_previous_workspace())),
        ("M-h", modify_with(|cs| cs.next_screen())),
        ("M-l", modify_with(|cs| cs.previous_screen())),
        ("M-f", fullscreen()),
        ("M-Up", send_layout_message(|| IncMain(1))),
        ("M-Down", send_layout_message(|| IncMain(-1))),
        ("M-Right", send_layout_message(|| ExpandMain)),
        ("M-Left", send_layout_message(|| ShrinkMain)),
        ("A-h", modify_with(|cs| cs.next_layout())),
        ("A-l", modify_with(|cs| cs.previous_layout())),
        ("M-p", Box::new(gpass)),
        ("M-m", spawn("maim -s /home/me/Downloads/screenshot.png")),
        ("M-o", Box::new(terminal)),
        ("M-semicolon", menu::launch()),
        ("M-C-x", spawn("pkill -fi ferris")),
        ("M-C-r", exit()),
        (
            "XF86AudioLowerVolume",
            spawn("wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-"),
        ),
        (
            "XF86AudioRaiseVolume",
            spawn("wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+"),
        ),
        (
            "XF86AudioMute",
            spawn("wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle"),
        ),
        ("XF86MonBrightnessUp", spawn("brightnessctl set +10%")),
        ("XF86MonBrightnessDown", spawn("brightnessctl set 10%-")),
    ];

    let mut keys: HashMap<_, _> = bindings
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    for tag in &["1", "2", "3", "4", "5", "6"] {
        keys.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.pull_tag_to_screen(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    keys
}
