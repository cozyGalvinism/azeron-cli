use std::{num::Wrapping, fmt::Display};

use hidapi::HidDevice;

use super::{unsorted::StringOrU32, button::{ButtonType, Button}, statics::JAVASCRIPT_TO_DEVICE_KEY, key_manager::meta_key_to_device_code};

pub enum RequestMessage {
    /// Sends `Hi` to the Azeron device.
    KeepAlive,
    /// Sends `GET_FW_VERSION` to the Azeron device.
    ///
    /// Returns the firmware version as a string.
    ///
    /// Example: `FWV_60_0`
    GetFirmwareVersion,
    /// Sends `GET_LEDS` to the Azeron device.
    ///
    /// Returns the LED state as a string.
    ///
    /// Example: `LEDS_ON`
    GetLEDState,
    /// Sends `LEDBRT` to the Azeron device.
    ///
    /// Returns the LED brightness as a string.
    ///
    /// Example: `LEDBRT_5`
    GetLEDBrightness,
    /// Sends `GET_ANALOG_TYPE` to the Azeron device.
    ///
    /// Returns the analog type as a string. (`ANALOG_SQUARE` or `ANALOG_CIRCLE`)
    ///
    /// Example: `ANALOG_SQUARE`
    GetAnalogType,
    /// Sende `GET_PROFILES` to the Azeron device.
    ///
    /// Returns the profile list as a string.
    ///
    /// An example would be too long to include here.
    GetProfiles,
    /// Sends `ST|<id>` to the Azeron device.
    ///
    /// <id> is the button ID. Returns the switch state as a string.
    ///
    /// Example: `ST0_21` (button 21, state 0)
    GetSwitchState { id: u8 },
    /// Sends `GET_FW_TYPE` to the Azeron device.
    ///
    /// Returns the keypad type as a string.
    ///
    /// Example: `FWT_1_3_0_0`
    GetKeypadInfo,
    /// Sends `BTNTHT` to the Azeron device.
    ///
    /// Returns the button throttle as a string.
    ///
    /// Example: `BTNTHT_10`
    GetButtonThrottle,
    /// Sends `GET_THROTTLE_TYPE` to the Azeron device.
    ///
    /// Returns the throttle type as a string.
    ///
    /// Example: `THROTTLE_TYPE_1`
    GetThrottleType,
    /// Sends `HWANLGOFST` to the Azeron device.
    ///
    /// Returns the hardware analog offset as a string.
    ///
    /// Example: `HWANLGOFST_-5_-8`
    GetHardwareAnalogOffset,
    /// Sends `HWLWRDZ` to the Azeron device.
    ///
    /// Returns the lower hardware deadzone as a string.
    ///
    /// Example: `HWLWRDZ_0`
    GetHardwareLowerDeadzone,
    /// Sends `HWUPRDZ` to the Azeron device.
    ///
    /// Returns the upper hardware deadzone as a string.
    ///
    /// Example: `HWUPRDZ_298`
    GetHardwareUpperDeadzone,
    /// Sends `GET_RIGHT_ANALOG` to the Azeron device.
    ///
    /// Returns the right analog setting as a string.
    GetRightAnalogSetting,
    SetButton {
        profile_id: u32,
        button: Button,
        is_js_keycode: bool,
    },
    /// Sends a custom command to the Azeron device.
    ///
    /// This may or may not return a response, so only use this if you know what you're doing.
    Custom(String),
}

pub enum ResponseMessage {
    KeepAlive(Vec<u8>),
    SetButtonResponse(bool),
    None,
}

impl RequestMessage {
    pub fn send_message(&self, device: &HidDevice) -> Result<ResponseMessage, ()> {
        match self {
            RequestMessage::KeepAlive => {
                let msg = to_azeron_message("Hi").unwrap();
                device.write(&msg).unwrap();
                let mut packet = [0u8; 64];
                device.read(&mut packet).unwrap();
                println!("{:?}", packet);
                let mut slice = packet[9..9 + packet[8] as usize].to_vec();
                let message_type = packet[4];
                handle_hid_payload(message_type, &mut slice);

                Ok(ResponseMessage::KeepAlive(slice))
            }
            RequestMessage::GetFirmwareVersion => todo!(),
            RequestMessage::GetLEDState => todo!(),
            RequestMessage::GetLEDBrightness => todo!(),
            RequestMessage::GetAnalogType => todo!(),
            RequestMessage::GetProfiles => todo!(),
            RequestMessage::GetSwitchState { id } => todo!(),
            RequestMessage::GetKeypadInfo => todo!(),
            RequestMessage::GetButtonThrottle => todo!(),
            RequestMessage::GetThrottleType => todo!(),
            RequestMessage::GetHardwareAnalogOffset => todo!(),
            RequestMessage::GetHardwareLowerDeadzone => todo!(),
            RequestMessage::GetHardwareUpperDeadzone => todo!(),
            RequestMessage::GetRightAnalogSetting => todo!(),
            RequestMessage::SetButton {
                profile_id,
                button,
                is_js_keycode,
            } => {
                let mut keys = button.key_values.iter().enumerate().map(|(index, key_value)| {
                    let key_value = match key_value {
                        StringOrU32::String(s) => s.parse().unwrap(),
                        StringOrU32::U32(u) => *u,
                    };
                    
                    if button.button_type == ButtonType::MouseButton || button.button_type == ButtonType::JoystickButton || 
                    button.button_type == ButtonType::XInputButton || button.button_type == ButtonType::JoystickHat ||
                    button.button_type == ButtonType::XInputTrigger || button.button_type == ButtonType::SwitchProfile ||
                    ((button.is_analog_joystick() || button.button_type == ButtonType::None) && index == 3) {
                        key_value.to_string()
                    } else if *is_js_keycode {
                        JAVASCRIPT_TO_DEVICE_KEY.get(&key_value).unwrap().to_string()
                    } else {
                        key_value.to_string()
                    }
                })
                .collect::<Vec<String>>();
                keys.resize(4, "0".to_string());
                let keys = keys.join("|");
                let mut meta_keys = button.meta_keys.iter().map(|key| {
                    let key = match key {
                        StringOrU32::String(s) => meta_key_to_device_code(s),
                        StringOrU32::U32(u) => *u,
                    };
                    
                    key.to_string()
                })
                .collect::<Vec<String>>();
                meta_keys.resize(3, "0".to_string());
                let meta_keys = meta_keys.join("|");
                let msg_str = format!("B{}|{}|{}|{}|{}|{}|{}|0", profile_id, button.id, u8::from(&button.button_type), button.pins[0], button.pins[1], keys, meta_keys);
                let msg = to_azeron_message(&msg_str).unwrap();
                device.write(&msg).unwrap();
                let mut packet = [0u8; 64];
                device.read(&mut packet).unwrap();
                let device_response = String::from_utf8(packet.to_vec());
                if let Err(e) = device_response {
                    println!("Error: {:?}", e);
                    return Err(());
                }
                let device_response = device_response.unwrap();

                Ok(ResponseMessage::SetButtonResponse(device_response.starts_with(format!("BOK_{}", button.id).as_str())))
            },
            RequestMessage::Custom(msg) => todo!(),
        }
    }
}

impl Display for ResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResponseMessage::KeepAlive(msg) => write!(f, "{:?}", msg),
            ResponseMessage::None => write!(f, ""),
            ResponseMessage::SetButtonResponse(success) => write!(f, "{}", success),
        }
    }
}

fn is_set(byte: u8, bytes: &[u8]) -> bool {
    let index = 5 - (byte - 1) / 8;
    0 != (bytes[index as usize] & 1 << ((byte - 1) % 8))
}

fn to_azeron_message(message: &str) -> Result<Vec<u8>, ()> {
    // ensure message contains only ascii characters
    if message.chars().any(|c| !c.is_ascii()) {
        return Err(());
    }
    let bytes = message.as_bytes();
    // get length of bytes as ascii characters
    let length = bytes.len();
    // convert length to ascii characters
    let length_str = length.to_string();
    let length_bytes = length_str.as_bytes();

    // message is ^length_bytes~message\n
    let mut message_bytes = vec![0, b'^'];
    message_bytes.extend_from_slice(length_bytes);
    message_bytes.push(b'~');
    message_bytes.extend_from_slice(bytes);
    message_bytes.push(b'\n');

    Ok(message_bytes)
}

#[allow(arithmetic_overflow)]
fn handle_hid_payload(message_type: u8, payload: &mut [u8]) {
    if message_type != 1 {
        return;
    }

    #[derive(Debug)]
    struct Internal {
        x: Wrapping<u8>,
        y: Wrapping<u8>,
    }
    let i = Internal {
        x: Wrapping(payload[11]) << 8 | Wrapping(payload[10]),
        y: Wrapping(payload[13]) << 8 | Wrapping(payload[12]),
    };
    let o = Internal {
        x: Wrapping(payload[7]) << 8 | Wrapping(payload[6]),
        y: Wrapping(payload[9]) << 8 | Wrapping(payload[8]),
    };
    println!("{:?}", i);
    println!("{:?}", o);

    // for i in 0..38 {
    //     let j = i + 1;
    //     if !((24..=27).contains(&j) || (32..=35).contains(&j)) {
    //         let is_set = if is_set(j, payload) { 1 } else { 0 };
    //         for profile in profiles.iter_mut() {
    //             if let Some(button) = profile.buttons.get_mut(j as usize) {
    //                 button.state = is_set;
    //             }
    //         }
    //     }
    // }

    // for profile in profiles.iter_mut() {

    // }
}