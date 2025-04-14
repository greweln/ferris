use std::fs;

// Function to read a system file
pub fn read_sys_file(file: &str) -> String {
    fs::read_to_string(format!("{}", file))
        .unwrap_or_else(|_| "!?".to_owned())
        .trim()
        .to_string()
}
