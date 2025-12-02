use crate::{COLORS, DMENU_FONT};
use penrose::{Color, Result, util::spawn_with_args};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

// Wrapper around dmenu
pub struct DMenu {
    options: Vec<String>,
    flags: Flags,
}

impl DMenu {
    pub fn new(options: Vec<String>, flags: Flags) -> Self {
        Self { options, flags }
    }

    // Run 'dmenu_run' command
    pub fn dmenu_run() -> Result<()> {
        let configs = Flags::default().flags();
        let configs_ref: Vec<&str> = configs.iter().map(|f| f.as_str()).collect();
        spawn_with_args("dmenu_run", &configs_ref)?;
        Ok(())
    }

    // Pipe a list of options into dmenu and return the selected one
    pub fn run(&self) -> Result<String> {
        let options = self.options.join("\n").as_bytes().to_vec();
        let flags = self.flags.flags();

        let mut proc = Command::new("dmenu")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(flags)
            .spawn()?;

        {
            // write options to stdin
            let mut stdin = proc.stdin.take().unwrap();
            stdin.write_all(&options)?;
        } // <--- critical: stdin is dropped here (EOF to dmenu)

        let mut choice = String::new();
        proc.stdout.take().unwrap().read_to_string(&mut choice)?;

        Ok(choice.trim().to_string())
    }
}

#[derive(Clone)]
pub struct Flags {
    pub ignore_case: bool,
    pub nb: Color,
    pub nf: Color,
    pub sb: Color,
    pub sf: Color,
    pub lines: u8,
    pub font: String,
    pub bottom: bool,
    pub prompt: String,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            prompt: "=>".to_string(),
            nb: COLORS.black,
            nf: COLORS.foreground,
            sb: COLORS.gray,
            sf: COLORS.white,
            lines: 10,
            font: DMENU_FONT.to_string(),
            ignore_case: true,
            bottom: true,
        }
    }
}

impl Flags {
    fn flags(&self) -> Vec<String> {
        let mut flags = vec![
            "-nb".to_owned(),
            self.nb.as_rgb_hex_string(),
            "-nf".to_owned(),
            self.nf.as_rgb_hex_string(),
            "-sb".to_owned(),
            self.sb.as_rgb_hex_string(),
            "-sf".to_owned(),
            self.sf.as_rgb_hex_string(),
            "-fn".to_owned(),
            self.font.to_string(),
            "-l".to_owned(),
            self.lines.to_string(),
            "-p".to_owned(),
            self.prompt.to_owned(),
            "-c".to_owned(),
        ];

        if self.ignore_case {
            flags.push("-i".to_owned());
        }

        if self.bottom {
            flags.push("-b".to_owned());
        }

        flags
    }
}
