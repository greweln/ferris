use once_cell::sync::Lazy;
use utils::resources::Xresources;

pub static COLORS: Lazy<Xresources> = Lazy::new(|| Xresources::colors());

pub mod bar;
pub mod errors;
pub mod keys;
pub mod menu;
pub mod utils;

pub type FerrisResult<T> = std::result::Result<T, errors::FerrisErrors>;

pub const STARTUP_SCRIPT: &str = "/home/me/.config/ferris/start.sh";
pub const TERMINAL: &str = "st";
pub const BROWSER: &str = "firefox";

// Windows
pub const MAX_MAIN: u32 = 1;
pub const RATIO: f32 = 0.6;
pub const RATIO_CENTER: f32 = 0.5;
pub const RATIO_STEP: f32 = 0.1;
pub const OUTER_PX: u32 = 0;
pub const INNER_PX: u32 = 0;

// Bar
pub const BAR_HEIGHT_PX: u32 = 22;
pub const FONT: &str = "terminus";
pub const FONT_SIZE: u8 = 12;

// Dmenu
const DMENU_FONT: &str = "TerminessNerdFont-18";
