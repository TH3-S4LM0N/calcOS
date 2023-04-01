use crate::vga_buffer::{Color, ColorCode, DEFAULT_COLOR, WRITER};
use core::fmt;

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if cfg!(feature = "log") {
            $crate::log::__log($crate::vga_buffer::Color::Green, module_path!(), "INFO", format_args!($($arg)*));
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(feature = "log") && cfg!(debug_assertions) {
            $crate::log::__log($crate::vga_buffer::Color::LightBlue, module_path!(), "DEBUG", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if cfg!(feature = "log") {
            $crate::log::__log($crate::vga_buffer::Color::Red, module_path!(), "ERROR", format_args!($($arg)*));
        }
    }
}

#[cfg(feature = "log")]
pub fn __log(color: Color, mod_path: &str, level: &str, msg: fmt::Arguments) {
    WRITER
        .lock()
        .change_color(ColorCode::new(color, Color::Black));
    crate::vga_buffer::__print(format_args!("{mod_path} {level}: {msg}\n"));
    WRITER.lock().change_color(DEFAULT_COLOR);
}
