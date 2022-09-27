use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzeronMacro {
    pub repeat: bool,
    pub steps: Vec<AzeronMacroStep>,
    #[serde(skip)]
    pub current_step: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzeronMacroStep {
    #[serde(rename = "type")]
    pub macro_type: MacroStepType,
    pub value: String,
    pub key_code: u32,
    pub hold_down_for: u32,
    #[serde(skip)]
    pub time_out_handle: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MacroStepType {
    Timeout,
    ButtonPress,
    ButtonDown,
    MouseDown,
    MouseUp,
    MouseClick,
}