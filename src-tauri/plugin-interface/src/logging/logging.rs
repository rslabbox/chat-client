#[repr(u8)]
#[allow(dead_code)]
pub enum ColorCode {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

#[allow(dead_code)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

#[macro_export]
macro_rules! with_color {
    ($color_code:expr, $($arg:tt)*) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[m", $color_code as u8, format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! host_log_print {
    ($level:expr, $msg:expr) => {
        {
            let level_color = match $level {
                $crate::LogLevel::Error => $crate::ColorCode::BrightRed,
                $crate::LogLevel::Warn => $crate::ColorCode::BrightYellow,
                $crate::LogLevel::Info => $crate::ColorCode::BrightGreen,
                $crate::LogLevel::Debug => $crate::ColorCode::BrightCyan,
                $crate::LogLevel::Trace => $crate::ColorCode::BrightBlack,
            };
            let args_color = match $level {
                $crate::LogLevel::Error => $crate::ColorCode::Red,
                $crate::LogLevel::Warn => $crate::ColorCode::Yellow,
                $crate::LogLevel::Info => $crate::ColorCode::Green,
                $crate::LogLevel::Debug => $crate::ColorCode::Cyan,
                $crate::LogLevel::Trace => $crate::ColorCode::BrightBlack,
            };
            println!(
                "[{} {} {}",
                $crate::with_color!(level_color, "{:<5}", $level),
                $crate::with_color!($crate::ColorCode::White, "{}:{}]", file!(), line!()),
                $crate::with_color!(args_color, "{}", $msg)
            );
        }
    };

    ($level:expr, $fmt:expr, $($args:expr),*) => {
        {
            let level_color = match $level {
                $crate::LogLevel::Error => $crate::ColorCode::BrightRed,
                $crate::LogLevel::Warn => $crate::ColorCode::BrightYellow,
                $crate::LogLevel::Info => $crate::ColorCode::BrightGreen,
                $crate::LogLevel::Debug => $crate::ColorCode::BrightCyan,
                $crate::LogLevel::Trace => $crate::ColorCode::BrightBlack,
            };
            let args_color = match $level {
                $crate::LogLevel::Error => $crate::ColorCode::Red,
                $crate::LogLevel::Warn => $crate::ColorCode::Yellow,
                $crate::LogLevel::Info => $crate::ColorCode::Green,
                $crate::LogLevel::Debug => $crate::ColorCode::Cyan,
                $crate::LogLevel::Trace => $crate::ColorCode::BrightBlack,
            };
            let msg = format!($fmt, $($args),*);
            println!(
                "[{} {} {}",
                $crate::with_color!(level_color, "{:<5}", $level),
                $crate::with_color!($crate::ColorCode::White, "{}:{}]", file!(), line!()),
                $crate::with_color!(args_color, "{}", msg)
            );
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($fmt:expr) => {
        $crate::host_log_print!($crate::LogLevel::Error, $fmt)
    };
    ($fmt:expr, $($args:expr),*) => {
        $crate::host_log_print!($crate::LogLevel::Error, $fmt, $($args),*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($fmt:expr) => {
        $crate::host_log_print!($crate::LogLevel::Warn, $fmt)
    };
    ($fmt:expr, $($args:expr),*) => {
        $crate::host_log_print!($crate::LogLevel::Warn, $fmt, $($args),*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($fmt:expr) => {
        $crate::host_log_print!($crate::LogLevel::Info, $fmt)
    };
    ($fmt:expr, $($args:expr),*) => {
        $crate::host_log_print!($crate::LogLevel::Info, $fmt, $($args),*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($fmt:expr) => {
        $crate::host_log_print!($crate::LogLevel::Debug, $fmt)
    };
    ($fmt:expr, $($args:expr),*) => {
        $crate::host_log_print!($crate::LogLevel::Debug, $fmt, $($args),*)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($fmt:expr) => {
        $crate::host_log_print!($crate::LogLevel::Trace, $fmt)
    };
    ($fmt:expr, $($args:expr),*) => {
        $crate::host_log_print!($crate::LogLevel::Trace, $fmt, $($args),*)
    };
}
