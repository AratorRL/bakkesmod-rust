use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BakkesModError(String);
impl Error for BakkesModError {}

impl BakkesModError {
    pub fn new(msg: &str) -> BakkesModError {
        Self(msg.to_owned())
    }
}

impl fmt::Display for BakkesModError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_export]
macro_rules! bakkesmod_error {
    ($($arg:tt)*) => ({
        log_console!("Error: {}", &format!($($arg)*));
        Err(Box::new(BakkesModError::new(&format!($($arg)*))))
    })
}
