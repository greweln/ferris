pub mod bar;
pub mod errors;
pub mod keys;
pub mod util;
pub mod widgets;

pub type FerrisResult<T> = std::result::Result<T, errors::FerrisErrors>;

pub const STARTUP_SCRIPT: &str = "/home/me/.config/ferris/start.sh";
pub const TERMINAL: &str = "st";

// COLORS
pub const WHITE: u32 = 0x696969ff;
pub const WHITE_BRIGHT: u32 = 0xc0c0c0ff;
pub const RED: u32 = 0x8c4665ff;

// Windows
pub const BORDER_NORMAL: u32 = 0x222222ff;
pub const BORDER_SELECTED: u32 = 0x696969ff;
pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.6;
pub const RATIO_CENTER: f32 = 0.5;
pub const RATIO_STEP: f32 = 0.1;
pub const OUTER_PX: u32 = 0;
pub const INNER_PX: u32 = 0;

// Bar
pub const BAR_HEIGHT_PX: u32 = 28;
pub const BAR_BACKGROUND: u32 = 0x262626ff;
pub const FONT: &str = "TerminessNerdFont-15";
pub const FONT_SIZE: u8 = 18;

// Workspaces
pub const WS_HIGHLIGHT: u32 = 0x899ca1ff; // BG => focused screen, FG => unfocused screens
pub const WS_FG: u32 = 0x515151ff; // FG => ws with windows not on monitors.(except focused monitor)
pub const WS_BG: u32 = 0x121212ff; // BG => all ws except the focused monitor.
