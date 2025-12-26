use crate::{
    BROWSER, TERMINAL,
    utils::dmenu::{DMenu, Flags},
};
use penrose::{
    Error::Custom,
    Result,
    builtin::actions::key_handler,
    core::bindings::KeyEventHandler,
    extensions::util::{NotifyLevel, notify_send, notify_send_custom},
    util::{spawn, spawn_with_args},
    x11rb::RustConn,
};

use std::io::Write;
use std::process::{Command, Stdio};

use std::thread;
use std::time::Duration;
pub fn launch() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|_, _| {
        let flags = Flags::default();

        let mut menu: Vec<String> = [
            "Timer", "Browser", "GPass", "Joplin", "SSH", "Yazi", "Calcurse", "Power", "Icons",
            "System",
        ]
        .into_iter()
        .map(|i| i.to_owned())
        .collect();

        menu.sort();

        let dmenu = DMenu::new(menu, flags);

        if let Ok(val) = dmenu.run() {
            match val.as_str().trim() {
                "Timer" => timer(),
                "Browser" => spawn(BROWSER),
                "Joplin" => spawn_with_args("st-joplin", &["-e", "joplin"]),
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

/// Timer function with inlined input handling
fn timer() -> Result<()> {
    /// Convert input like "10s", "5m", "1h" to seconds
    fn parse_time(input: &str) -> Result<u64> {
        let input = input.trim().to_lowercase();
        if input.is_empty() {
            return Err(Custom("Empty time string".to_string()));
        }

        let last_char = input.chars().last().unwrap();
        let (num_str, multiplier) = match last_char {
            's' => (&input[..input.len() - 1], 1),
            'm' => (&input[..input.len() - 1], 60),
            'h' => (&input[..input.len() - 1], 3600),
            '0'..='9' => (input.as_str(), 1), // default seconds
            _ => return Err(Custom("Invalid time unit".to_string())),
        };

        let num: u64 = num_str
            .parse()
            .map_err(|_| Custom("Invalid number".to_string()))?;
        Ok(num * multiplier)
    }

    let mut flags = Flags::default();

    // --- TIME INPUT ---
    flags.prompt = "Time (e.g. 10s, 5m, 1h):".into();
    let time_input = match DMenu::new(vec![], flags.clone()).run() {
        Ok(t) if t.trim().is_empty() => {
            notify_send("Timer", "No time value. Exiting")?;
            return Ok(());
        }
        Ok(t) => t,
        Err(_) => return Ok(()), // ESC → exit silently
    };

    let time_secs = match parse_time(&time_input) {
        Ok(secs) if secs > 0 => secs,
        _ => {
            notify_send(
                "Timer",
                "Timer must be a positive integer or valid time string",
            )?;
            return Ok(());
        }
    };

    // --- MESSAGE INPUT ---
    flags.prompt = "Message:".into();
    let message = match DMenu::new(vec![], flags).run() {
        Ok(m) if m.trim().is_empty() => "Time's up!".to_string(),
        Ok(m) => m,
        Err(_) => "Time's up!".to_string(), // ESC → default message
    };

    // --- SPAWN BACKGROUND TIMER ---
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(time_secs));
        let _ = Command::new("notify-send")
            .arg("-u")
            .arg("critical")
            .arg("Timer")
            .arg(&message)
            .status();
    });

    Ok(())
}

fn icons() -> Result<()> {
    let icons = [
        "󱓻 ",
        " ",
        "",
        ">",
        ">",
    ]
    .into_iter()
    .map(|i| i.to_owned())
    .collect();

    let dmenu = DMenu::new(icons, Flags::default());

    if let Ok(val) = dmenu.run() {
        // echo "$val" | xclip -selection clipboard
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(Stdio::piped()) // same as: ... | xclip
            .spawn()
            .expect("Failed to start xclip");

        // Write the text into xclip's stdin
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(val.as_bytes())?;
        }
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
    let menu = vec!["Poweroff", "Reboot", "Lock", "Sleep", "Logout", "Hibernate"];
    let mut options: Vec<String> = menu.iter().map(|i| i.to_string()).collect();
    options.sort();
    let flags = Flags::default();
    let dmenu = DMenu::new(options, flags);

    if let Ok(val) = dmenu.run() {
        match val.as_str().trim() {
            "Poweroff" => spawn("sudo poweroff"),
            "Reboot" => spawn("sudo reboot"),
            "Sleep" => spawn("systemctl suspend"),
            "Hiberante" => spawn("systemctl hibernate"),
            "Lock" => spawn("loginctl lock-session"),
            "Logout" => spawn("sudo pkill -KILL -u me"),
            _ => Ok(()),
        }
    } else {
        Ok(())
    }
}
