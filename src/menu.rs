use crate::{
    BROWSER, TERMINAL,
    utils::dmenu::{DMenu, Flags},
};
use penrose::{
    Result,
    builtin::actions::key_handler,
    core::bindings::KeyEventHandler,
    util::{spawn, spawn_with_args},
    x11rb::RustConn,
};

pub fn launch() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|_, _| {
        let flags = Flags::default();

        let menu = [
            "Browser", "Joplin", "SSH", "Yazi", "Calcurse", "Icons", "Cmus", "Power", "System",
        ]
        .into_iter()
        .map(|i| i.to_owned())
        .collect();

        let dmenu = DMenu::new(menu, flags);

        if let Ok(val) = dmenu.run() {
            match val.as_str().trim() {
                "Browser" => spawn(BROWSER),
                "Joplin" => spawn_with_args(TERMINAL, &["-e", "joplin"]),
                "Yazi" => spawn_with_args(TERMINAL, &["-e", "yazi"]),
                "Calcurse" => spawn_with_args(TERMINAL, &["-e", "calcurse"]),
                // "Icons" => icons(),
                "Cmus" => spawn_with_args(TERMINAL, &["-e", "cmus"]),
                // "SSH" => ssh(),
                "Power" => power(),
                "System" => DMenu::dmenu_run(),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    })
}

fn power() -> Result<()> {
    let menu = vec!["Poweroff", "Reboot", "Lock"];
    let options = menu.iter().map(|i| i.to_string()).collect();
    let flags = Flags::default();
    let dmenu = DMenu::new(options, flags);

    if let Ok(val) = dmenu.run() {
        match val.as_str().trim() {
            "Poweroff" => spawn("sudo poweroff"),
            "Reboot" => spawn("sudo reboot"),
            "Lock" => spawn("slock"),
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}
