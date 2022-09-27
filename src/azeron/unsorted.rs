use std::{collections::HashMap, fmt::Display, hash::Hash, num::Wrapping};

use chrono::Utc;
use hidapi::HidDevice;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{macros::AzeronMacro, button::{ButtonState, ButtonType, Button}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrU32 {
    String(String),
    U32(u32),
}

impl From<u32> for StringOrU32 {
    fn from(u: u32) -> Self {
        StringOrU32::U32(u)
    }
}

impl From<String> for StringOrU32 {
    fn from(s: String) -> Self {
        StringOrU32::String(s)
    }
}

impl Display for StringOrU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrU32::String(s) => write!(f, "{}", s),
            StringOrU32::U32(u) => write!(f, "{}", u),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub buttons: Vec<Button>,
    #[serde(skip)]
    pub is_active: bool,
    pub is_favourite: bool,
    pub is_software: bool,
    #[serde(skip)]
    pub favourited_at: Option<String>,
    #[serde(skip)]
    pub is_on_pressed_switch: bool,
    pub hash: String,
    pub id: u32,
    pub name: String,
}

impl Profile {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            buttons: vec![],
            is_active: false,
            is_favourite: false,
            is_software: true,
            favourited_at: None,
            is_on_pressed_switch: false,
            hash: "".to_string(),
            name: name.to_string(),
            id,
        }
    }

    pub fn add_button(&mut self, button: Button) {
        for i in 0..self.buttons.len() {
            if self.buttons[i].id == button.id {
                self.buttons[i] = button;
                return;
            }
        }
        self.buttons.push(button);
    }

    pub fn favourite(&mut self) {
        self.is_favourite = true;
        self.favourited_at = Some(Utc::now().to_rfc3339());
    }

    pub fn unfavourite(&mut self) {
        self.is_favourite = false;
        self.favourited_at = None;
    }

    pub fn is_switch_down(&self) -> bool {
        self.buttons[21 as usize].state == ButtonState::Pressed
    }

    pub fn calculate_hash(&self) -> String {
        format!(
            "{:02x}",
            md5::compute(
                self.buttons
                    .iter()
                    .map(|b| b.key_name())
                    .collect::<Vec<_>>()
                    .join("-")
                    .as_bytes()
            )
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoystickZone {
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_profile_hash() {
        let example_file = std::fs::read_to_string("example-profile.json").unwrap();
        let profile: super::Profile = serde_json::from_str(&example_file).unwrap();
        let hash = profile.calculate_hash();

        assert_eq!(hash, "265cdd71db25f5a9e518d39e6bca2a72");
    }
}
