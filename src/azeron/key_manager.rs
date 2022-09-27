use super::{AzeronCliError, statics::{REVERSED_DEVICE_KEYS, JAVASCRIPT_KEY_CODES, DEVICE_KEY_TO_JAVASCRIPT, REVERSED_JAVASCRIPT_KEY_CODES, JAVASCRIPT_TO_DEVICE_KEY, DEVICE_KEYS, JAVASCRIPT_KEY_CODE_TO_ROBOT}};

pub fn key_name_to_device_value(key_name: &str) -> Result<u32, AzeronCliError> {
    if key_name.contains(" + ") {
        return Err(AzeronCliError::KeyCombinationsNotSupported);
    }
    if is_modifier_key(key_name) {
        return Ok(*REVERSED_DEVICE_KEYS.get(format!("MODIFIERKEY_LEFT_{}", key_name).as_str()).unwrap())
    }
    if is_normal_key(key_name) {
        return Ok(*REVERSED_DEVICE_KEYS.get(format!("KEY_{}", key_name).as_str()).unwrap())
    }

    Err(AzeronCliError::KeyNotSupported { key_name: key_name.to_string() })
}

pub fn is_modifier_key(key_name: &str) -> bool {
    REVERSED_DEVICE_KEYS.contains_key(format!("MODIFIERKEY_LEFT_{}", key_name).as_str())
}

pub fn is_normal_key(key_name: &str) -> bool {
    REVERSED_DEVICE_KEYS.contains_key(format!("KEY_{}", key_name).as_str())
}

pub fn key_code_to_name(key_code: u32) -> Result<&'static str, AzeronCliError> {
    if key_code == 0 {
        return Ok("none");
    }

    if let Some(key_name) = JAVASCRIPT_KEY_CODES.get(&key_code) {
        return Ok(key_name);
    }

    Err(AzeronCliError::UndefinedKeyName { key_code })
}

pub fn device_code_to_js_key_code(device_code: &str) -> Result<u32, AzeronCliError> {
    if let Some(key_code) = DEVICE_KEY_TO_JAVASCRIPT.get(&device_code) {
        return Ok(*key_code);
    }

    Err(AzeronCliError::UndefinedKeyCode { key_name: device_code.to_string() })
}

pub fn js_key_code_to_device_code(key_code: u32) -> Result<&'static str, AzeronCliError> {
    if let Some(device_code) = JAVASCRIPT_KEY_CODES.get(&key_code) {
        return Ok(device_code);
    }

    Err(AzeronCliError::UndefinedKeyCode { key_name: key_code.to_string() })
}

pub fn key_name_to_js_key_code(key_name: &str) -> Result<u32, AzeronCliError> {
    if let Some(key_code) = REVERSED_JAVASCRIPT_KEY_CODES.get(key_name) {
        return Ok(*key_code);
    }

    Err(AzeronCliError::UndefinedKeyCode { key_name: key_name.to_string() })
}

pub fn is_supported_on_hardware(key_code: u32) -> bool {
    JAVASCRIPT_TO_DEVICE_KEY.get(&key_code).is_some()
}

pub fn from_device_meta_key(key_name: &str) -> Result<String, AzeronCliError> {
    if key_name == "0" {
        return Ok("NONE".to_string());
    }
    let key = DEVICE_KEYS.get(&(key_name.parse::<i32>().unwrap() as u32));
    if let Some(key) = key {
        if key.contains("_ALT") {
            return Ok("ALT".to_string());
        }
        if key.contains("_CTRL") {
            return Ok("CTRL".to_string());
        }
        if key.contains("_SHIFT") {
            return Ok("SHIFT".to_string());
        }
    }
    Err(AzeronCliError::UnknownMetaKey { key_name: key_name.to_string() })
}

pub fn from_js_key_code_to_robot_key(js_key_code: u32) -> Result<&'static str, AzeronCliError> {
    if let Some(device_code) = JAVASCRIPT_KEY_CODE_TO_ROBOT.get(&js_key_code) {
        return Ok(device_code);
    }

    Err(AzeronCliError::UndefinedKeyCode { key_name: js_key_code.to_string() })
}

pub fn meta_key_to_device_code(meta_key: &str) -> u32 {
    match meta_key.to_uppercase().as_str() {
        "CTRL" => *REVERSED_DEVICE_KEYS.get("MODIFIERKEY_LEFT_CTRL").unwrap(),
        "ALT" => *REVERSED_DEVICE_KEYS.get("MODIFIERKEY_LEFT_ALT").unwrap(),
        "SHIFT" => *REVERSED_DEVICE_KEYS.get("MODIFIERKEY_LEFT_SHIFT").unwrap(),
        "SUPER" => *REVERSED_DEVICE_KEYS.get("MODIFIERKEY_LEFT_GUI").unwrap(),
        _ => 0
    }
}