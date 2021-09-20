use alloc::string::String;

use crate::command::constants::LOGIN_COMMAND;

/// Common commands for RouterOS API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandWord {
    /// [Login](https://help.mikrotik.com/docs/display/ROS/API#API-Initiallogin) command
    Login,
    /// Raw command to be sent as is
    Raw(String),
}

impl<'a> From<&'a CommandWord> for &'a str {
    fn from(value: &'a CommandWord) -> Self {
        match value {
            CommandWord::Login => LOGIN_COMMAND,
            CommandWord::Raw(v) => v.as_str(),
        }
    }
}

impl From<&str> for CommandWord {
    fn from(value: &str) -> Self {
        match value {
            LOGIN_COMMAND => CommandWord::Login,
            v => CommandWord::Raw(v.to_string()),
        }
    }
}
