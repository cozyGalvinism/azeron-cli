use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use super::{unsorted::{JoystickZone, StringOrU32}, macros::AzeronMacro, statics::XINPUT_TO_READABLE};

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ButtonState {
    Pressed = 0,
    Released = 1,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Released
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum ButtonType {
    KeyboardKey = 1,
    Switch = 2,
    AnalogJoystick = 3,
    AnalogJoystickWithKeys = 4,
    JoystickButton = 5,
    Disabled = 6,
    AnalogJoystickWithKeysUp = 7,
    AnalogJoystickWithKeysRight = 8,
    AnalogJoystickWithKeysDown = 9,
    AnalogJoystickWithKeysLeft = 10,
    None = 11,
    JoystickHat = 12,
    ToggleAnalog = 13,
    ToggleAnalogShort = 14,
    MouseButton = 15,
    Macro = 16,
    AnalogJoystickAndWithKeysUp = 17,
    AnalogJoystickAndWithKeysRight = 18,
    AnalogJoystickAndWithKeysDown = 19,
    AnalogJoystickAndWithKeysLeft = 20,
    XInputJoystick = 21,
    XInputButton = 22,
    XInputTrigger = 23,
    SwitchProfile = 24,
    AnalogJoystickWithDriftAndKeysUp = 25,
    AnalogJoystickWithDriftAndKeysRight = 26,
    AnalogJoystickWithDriftAndKeysDown = 27,
    AnalogJoystickWithDriftAndKeysLeft = 28,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::KeyboardKey
    }
}

impl From<ButtonType> for u8 {
    fn from(button_type: ButtonType) -> Self {
        button_type as u8
    }
}

impl From<&ButtonType> for u8 {
    fn from(button_type: &ButtonType) -> Self {
        *button_type as u8
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Button {
    pub pins: [u8; 2],
    pub key_values: Vec<StringOrU32>,
    pub meta_keys: Vec<StringOrU32>,
    pub button_type: ButtonType,
    pub user_label: String,
    #[serde(rename = "macro")]
    pub button_macro: AzeronMacro,
    pub id: u8,
    #[serde(skip)]
    pub state: ButtonState,
    #[serde(skip)]
    pub joystick: Option<JoystickZone>,
    #[serde(skip)]
    pub pure_joystick: Option<JoystickZone>,
}

impl Button {
    pub fn empty(id: u8) -> Self {
        Self {
            pins: [0, 0],
            key_values: vec![],
            meta_keys: vec![],
            button_type: ButtonType::KeyboardKey,
            joystick: Some(JoystickZone { x: 512, y: 512 }),
            pure_joystick: Some(JoystickZone { x: 512, y: 512 }),
            user_label: "".to_string(),
            id,
            state: ButtonState::Released,
            button_macro: AzeronMacro {
                repeat: false,
                steps: vec![],
                current_step: 0,
            },
        }
    }

    pub fn new(id: u8, button_type: ButtonType, pins: [u8; 2], key_values: Vec<StringOrU32>, meta_keys: Vec<StringOrU32>) -> Self {
        Self {
            pins,
            key_values,
            meta_keys,
            button_type,
            joystick: Some(JoystickZone { x: 512, y: 512 }),
            pure_joystick: Some(JoystickZone { x: 512, y: 512 }),
            user_label: "".to_string(),
            id,
            state: ButtonState::Released,
            button_macro: AzeronMacro {
                repeat: false,
                steps: vec![],
                current_step: 0,
            },
        }
    }

    pub fn key_name(&self) -> String {
        match self.button_type {
            ButtonType::KeyboardKey => todo!(),
            ButtonType::Switch => todo!(),
            ButtonType::AnalogJoystick => todo!(),
            ButtonType::AnalogJoystickWithKeys => todo!(),
            ButtonType::JoystickButton => format!("JOY_{}", self.key_values[0]),
            ButtonType::Disabled => "".to_string(),
            ButtonType::AnalogJoystickWithKeysUp => todo!(),
            ButtonType::AnalogJoystickWithKeysRight => todo!(),
            ButtonType::AnalogJoystickWithKeysDown => todo!(),
            ButtonType::AnalogJoystickWithKeysLeft => todo!(),
            ButtonType::None => todo!(),
            ButtonType::JoystickHat => todo!(),
            ButtonType::ToggleAnalog => "TGL RL ANALOG".to_string(),
            ButtonType::ToggleAnalogShort => "TGL HOLD RL ANALOG".to_string(),
            ButtonType::MouseButton => format!("M{}", self.key_values[0]),
            ButtonType::Macro => format!("MACRO ({})", self.button_macro.steps.len()),
            ButtonType::AnalogJoystickAndWithKeysUp => todo!(),
            ButtonType::AnalogJoystickAndWithKeysRight => todo!(),
            ButtonType::AnalogJoystickAndWithKeysDown => todo!(),
            ButtonType::AnalogJoystickAndWithKeysLeft => todo!(),
            ButtonType::XInputJoystick => todo!(),
            ButtonType::XInputButton => {
                let readable = match &self.key_values[0] {
                    StringOrU32::String(s) => XINPUT_TO_READABLE.get(&s.parse().unwrap()).unwrap(),
                    StringOrU32::U32(u) => XINPUT_TO_READABLE.get(u).unwrap(),
                };
                format!("XBOX {}", readable)
            },
            ButtonType::XInputTrigger => {
                let name = match &self.key_values[0] {
                    StringOrU32::String(s) => if s == "0" { "LT" } else { "RT" },
                    StringOrU32::U32(u) => if *u == 0 { "LT" } else { "RT" },
                };
                format!("XBOX {}", name)
            },
            ButtonType::SwitchProfile => "PROFILE".to_string(),
            ButtonType::AnalogJoystickWithDriftAndKeysUp => todo!(),
            ButtonType::AnalogJoystickWithDriftAndKeysRight => todo!(),
            ButtonType::AnalogJoystickWithDriftAndKeysDown => todo!(),
            ButtonType::AnalogJoystickWithDriftAndKeysLeft => todo!(),
        }
    }

    pub fn is_analog_joystick(&self) -> bool {
        self.button_type == ButtonType::AnalogJoystick ||
        self.button_type == ButtonType::XInputJoystick ||
        self.button_type == ButtonType::AnalogJoystickWithKeys ||
        self.button_type == ButtonType::AnalogJoystickWithKeysUp ||
        self.button_type == ButtonType::AnalogJoystickWithKeysRight ||
        self.button_type == ButtonType::AnalogJoystickWithKeysDown ||
        self.button_type == ButtonType::AnalogJoystickWithKeysLeft ||
        self.button_type == ButtonType::AnalogJoystickAndWithKeysUp ||
        self.button_type == ButtonType::AnalogJoystickAndWithKeysRight ||
        self.button_type == ButtonType::AnalogJoystickAndWithKeysDown ||
        self.button_type == ButtonType::AnalogJoystickAndWithKeysLeft ||
        self.button_type == ButtonType::AnalogJoystickWithDriftAndKeysUp ||
        self.button_type == ButtonType::AnalogJoystickWithDriftAndKeysRight ||
        self.button_type == ButtonType::AnalogJoystickWithDriftAndKeysDown ||
        self.button_type == ButtonType::AnalogJoystickWithDriftAndKeysLeft
    }
}