pub mod unsorted;
pub mod statics;
pub mod key_manager;
pub mod macros;
pub mod button;
pub mod communication;

custom_error::custom_error!(
    pub AzeronCliError
    KeyNotSupported { key_name: String } = "Key not supported: {key_name}",
    KeyCombinationsNotSupported = "Key combinations are not supported",
    UndefinedKeyName { key_code: u32 } = "Undefined key name for key code: {key_code}",
    UndefinedKeyCode { key_name: String } = "Undefined key code for key name: {key_name}",
    UnknownMetaKey { key_name: String } = "Unknown meta key: {key_name}",
);