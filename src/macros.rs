#[macro_export]
macro_rules! log_console {
    ($($arg:tt)*) => ({
        crate::bakkesmod::log(&format!($($arg)*));
    })
}
