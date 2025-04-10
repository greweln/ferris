use std::io::Read;
use std::process::{Command, Stdio};

// Run an external command with arguments and return its output.
/// > [`std::process::Command::output`] will not work within Penrose
pub fn run_command_for_output_with_args(cmd: String, args: &[&str]) -> std::io::Result<String> {
    let mut child = match Command::new(&cmd).args(args).stdout(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(err) => {
            let msg = format!("Error running: '{}' command. \n<{}>", cmd, err);
            notify(&msg, NotifyUrgencies::Low)?;
            return Ok("".to_string());
        }
    };

    let mut buff = String::new();

    child
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut buff)
        .map(|_| buff.trim().to_string())
}

/// Use 'notify-send' to display a message
pub enum NotifyUrgencies {
    Low,
    Normal,
    Critical,
}

impl NotifyUrgencies {
    fn level(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::Critical => "critical",
        }
    }
}

pub fn notify(msg: &str, urgency: NotifyUrgencies) -> std::io::Result<()> {
    Command::new("notify-send")
        .args(["-u", urgency.level(), msg])
        .output()
        .map(|_| ())
}
