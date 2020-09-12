#[macro_export]
macro_rules! log_console {
    ($($arg:tt)*) => ({
        $crate::console::console_print(&format!($($arg)*));
    })
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => (
        $crate::wrappers::structs::Vector::from($x as f32, $y as f32, $z as f32)
    );
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => (
        $crate::wrappers::structs::Vector2::from($x as i32, $y as i32)
    );
}

#[macro_export]
macro_rules! color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        $crate::wrappers::structs::Color::from($r as u8, $g as u8, $b as u8, $a as u8)
    );
}

#[macro_export]
macro_rules! lin_color {
    ($r:expr, $g:expr, $b:expr, $a:expr) => (
        $crate::wrappers::structs::LinearColor::from($r as f32, $g as f32, $b as f32, $a as f32)
    );
}