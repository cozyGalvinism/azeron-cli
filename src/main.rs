use azeron::{
    button::{Button, ButtonType},
    unsorted::StringOrU32, statics::BUTTON_IDS_TO_PINS,
};
use clap::{Parser, Subcommand};

mod azeron;

#[derive(Parser)]
struct CliArgs {
    #[clap(subcommand)]
    pub subcommand: CliAction,
}

#[derive(Subcommand)]
enum CliAction {
    /// Sets a button of the Azeron to the desired keys.
    SetButton {
        /// The profile to set the button in (0 or 1, other values will not work).
        profile_id: u32,
        /// The button ID to set (1-38), some of them cannot/shouldn't be set.
        button_id: u8,
        /// Meta keys to press (e.g. CTRL, ALT, SHIFT, SUPER). (optional, can be used multiple times for specifying multiple keys)
        #[clap(short, long = "meta-key")]
        meta_keys: Vec<String>,
        /// Key to press, using the Azeron key codes
        key_value: u32,
    }
}

fn main() {
    let args = CliArgs::parse();

    let api = hidapi::HidApi::new().expect("Failed to create HID API");
    let azeron = api
        .device_list()
        .find(|d| d.vendor_id() == 0x16d0 && d.product_id() == 0x10bc && d.interface_number() == 4)
        .expect("Failed to find Azeron device");
        let device = azeron.open_device(&api).expect("Failed to open Azeron device for communication");
    
    match args.subcommand {
        CliAction::SetButton { profile_id, button_id, key_value, meta_keys } => {
            let response = azeron::communication::RequestMessage::SetButton {
                profile_id,
                button: Button::new(
                    button_id,
                    ButtonType::KeyboardKey,
                    *BUTTON_IDS_TO_PINS.get(&button_id).expect("invalid button id"),
                    vec![key_value.into()],
                    meta_keys
                        .iter()
                        .map(|k| StringOrU32::String(k.to_string()))
                        .collect::<Vec<StringOrU32>>(),
                ),
                is_js_keycode: false,
            }
            .send_message(&device)
            .expect("Failed to send message");

            println!("success: {}", response);
        }
    }
}
