// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
#[must_use]
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    }
}
#[must_use]
pub fn info(message: &str) -> String {
    format!("[INFO]: {}", message)
}
#[must_use]
pub fn warn(message: &str) -> String {
    format!("[WARNING]: {}", message)
}
#[must_use]
pub fn error(message: &str) -> String {
    format!("[ERROR]: {}", message)
}

#[must_use]
pub fn debug(message: &str) -> String {
    format!("[DEBUG]: {}", message)
}
