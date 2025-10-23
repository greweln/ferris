use crate::{
    BROWSER, TERMINAL,
    utils::dmenu::{DMenu, Flags},
};
use penrose::{
    Result,
    builtin::actions::key_handler,
    core::bindings::KeyEventHandler,
    extensions::util::{NotifyLevel, notify_send_custom},
    util::{spawn, spawn_with_args},
    x11rb::RustConn,
};

pub fn launch() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|_, _| {
        let flags = Flags::default();

        let mut menu: Vec<String> = [
            "Browser", "GPass", "Joplin", "SSH", "Yazi", "Calcurse", "Power", "Icons", "System",
        ]
        .into_iter()
        .map(|i| i.to_owned())
        .collect();

        menu.sort();

        let dmenu = DMenu::new(menu, flags);

        if let Ok(val) = dmenu.run() {
            match val.as_str().trim() {
                "Browser" => spawn(BROWSER),
                "Joplin" => spawn_with_args(TERMINAL, &["-e", "joplin"]),
                "Yazi" => spawn_with_args(TERMINAL, &["-e", "yazi"]),
                "Calcurse" => spawn_with_args(TERMINAL, &["-e", "calcurse"]),
                "GPass" => spawn_with_args(TERMINAL, &["-e", "gpass", "/home/me/gpass.json"]),
                "SSH" => ssh(),
                "Icons" => icons(),
                "Power" => power(),
                "System" => DMenu::dmenu_run(),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    })
}

fn icons() -> Result<()> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let icons = ["󱓻 ", " ", "---------------------------------------------"]
        .into_iter()
        .map(|i| i.to_owned())
        .collect();

    let dmenu = DMenu::new(icons, Flags::default());

    if let Ok(val) = dmenu.run() {
        // Start xclip and open its stdin
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start xclip");

        // Write the text into xclip's stdin
        if let Some(stdin) = child.stdin.as_mut() {
            stdin
                .write_all(val.as_bytes())
                .expect("Failed to write to xclip");
        }

        // Wait for xclip to finish
        child.wait().expect("Failed to wait on xclip");
    }
    Ok(())
}

// Read and parse an ssh config file and display the hosts options in dmenu
fn ssh() -> Result<()> {
    use std::{collections::HashMap, fs, io::ErrorKind};

    #[derive(Debug)]
    struct Host {
        ip: String,
        user: String,
    }

    let content = match fs::read_to_string("/home/me/.ssh/config") {
        Ok(c) => c,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                notify_send_custom("Menu", "ssh config file not found", NotifyLevel::Low, 3000)?;
                return Ok(());
            } else {
                //  // Other IO errors: propagate
                return Err(e.into());
            }
        }
    };

    let mut conns: HashMap<String, Host> = HashMap::new();
    let mut conn_host: Option<Host> = None;
    let mut conn_name = String::new();

    for line in content.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(name) = line.strip_prefix("Host ") {
            // save previous host
            if let Some(host) = conn_host.take() {
                conns.insert(conn_name.clone(), host);
            }
            conn_name = name.to_string();
            conn_host = Some(Host {
                ip: String::new(),
                user: String::new(),
            });
        } else if let Some(host) = conn_host.as_mut() {
            if let Some((k, v)) = line.split_once(char::is_whitespace) {
                match k {
                    "HostName" => host.ip = v.trim().to_string(),
                    "User" => host.user = v.trim().to_string(),
                    _ => {}
                }
            }
        }
    }

    // save last host
    if let Some(host) = conn_host {
        conns.insert(conn_name, host);
    }

    let mut names: Vec<_> = conns.keys().cloned().collect();
    names.sort();

    let flags = Flags::default();

    let dmenu = DMenu::new(names, flags);

    if let Ok(selected) = dmenu.run() {
        if let Some(host) = conns.get(&selected) {
            let command = format!("ssh {}@{}", host.user, host.ip);
            spawn_with_args(TERMINAL, &["-e", "bash", "-l", "-c", command.as_str()])?;
        }
    }

    Ok(())
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
