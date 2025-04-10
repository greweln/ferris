use std::fmt;

// Implement my own error type to cmplicate my life ...

#[derive(Debug)]
pub enum FerrisErrors {
    WmError(penrose::Error),
    UiError(penrose_ui::Error),
}

// Implement Display for FerrisErrors
impl fmt::Display for FerrisErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FerrisErrors::WmError(e) => write!(f, "FerrisError (from penrose): {}", e),
            FerrisErrors::UiError(e) => write!(f, "FerrisUiError (from penrose_ui): {}", e),
        }
    }
}

// Implement From<penrose::Error> so I can convert automatically
impl From<penrose::Error> for FerrisErrors {
    fn from(err: penrose::Error) -> Self {
        FerrisErrors::WmError(err)
    }
}

// Implement From<penrose_ui::Error> so I can convert automatically
impl From<penrose_ui::Error> for FerrisErrors {
    fn from(err: penrose_ui::Error) -> Self {
        FerrisErrors::UiError(err)
    }
}
