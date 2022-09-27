# azeron-cli

A small, unfinished CLI application intended to manage the Azeron Cyborg.

The code is still in a very messy state and doesn't look very rusty yet, as the priority was to port it first and then transform the code to work better in Rust.

If you want to try out the current state, [you will need to install Rust first](https://rustup.rs). After you installed the latest stable version, you can run `cargo install --git https://github.com/cozyGalvinism/azeron-cli` to install it, although there may be bugs.

**Use this software at your own risk! It is not fully tested yet and might brick your Azeron. If you don't know how to reflash the firmware, do not use this yet! You have been warned!**

See the examples listed below in order to better understand how to use this software.

I am also not affiliated with Azeron in any way, shape or form. I am simply a Linux user, who wants to set Azeron keys on Linux without using a VM with some programming knowledge I accumulated.

Help with this project is greatly appreciated!

## Usage

Right now, only `set-button` is implemented, which will set a specified button to a keyboard key:

```text
USAGE:
    azeron-cli set-button [OPTIONS] <PROFILE_ID> <BUTTON_ID> <KEY_VALUE>

ARGS:
    <PROFILE_ID>    The profile to set the button in (0 or 1, other values will not work)
    <BUTTON_ID>     The button ID to set (1-38), some of them cannot/shouldn't be set
    <KEY_VALUE>     Key to press, using the Azeron key codes

OPTIONS:
    -h, --help                    Print help information
    -m, --meta-key <META_KEYS>    Meta keys to press (e.g. CTRL, ALT, SHIFT, SUPER). (optional, can
                                  be used multiple times for specifying multiple keys)
```

The button IDs can be taken from the official application, though here is an image of the layout from the official application:

![Button IDs](docs/assets/ids.png?raw=true)

Possible key values are as follows:

```text
57345 = "MODIFIERKEY_LEFT_CTRL"
57346 = "MODIFIERKEY_LEFT_SHIFT"
57348 = "MODIFIERKEY_LEFT_ALT"
57352 = "MODIFIERKEY_LEFT_GUI"
57360 = "MODIFIERKEY_RIGHT_CTRL"
57376 = "MODIFIERKEY_RIGHT_SHIFT"
57408 = "MODIFIERKEY_RIGHT_ALT"
57472 = "MODIFIERKEY_RIGHT_GUI"
57985 = "KEY_SYSTEM_POWER_DOWN"
57986 = "KEY_SYSTEM_SLEEP"
57987 = "KEY_SYSTEM_WAKE_UP"
58544 = "KEY_MEDIA_RANDOM_PLAY"
58545 = "KEY_MEDIA_PAUSE"
58546 = "KEY_MEDIA_RECORD"
58547 = "KEY_MEDIA_FAST_FORWARD"
58548 = "KEY_MEDIA_REWIND"
58549 = "KEY_MEDIA_NEXT_TRACK"
58550 = "KEY_MEDIA_PREV_TRACK"
58551 = "KEY_MEDIA_STOP"
58552 = "KEY_MEDIA_EJECT"
58573 = "KEY_MEDIA_PLAY_PAUSE"
58574 = "KEY_MEDIA_PLAY_SKIP"
58594 = "KEY_MEDIA_MUTE"
58601 = "KEY_MEDIA_VOLUME_INC"
58602 = "KEY_MEDIA_VOLUME_DEC"
61444 = "KEY_A"
61445 = "KEY_B"
61446 = "KEY_C"
61447 = "KEY_D"
61448 = "KEY_E"
61449 = "KEY_F"
61450 = "KEY_G"
61451 = "KEY_H"
61452 = "KEY_I"
61453 = "KEY_J"
61454 = "KEY_K"
61455 = "KEY_L"
61456 = "KEY_M"
61457 = "KEY_N"
61458 = "KEY_O"
61459 = "KEY_P"
61460 = "KEY_Q"
61461 = "KEY_R"
61462 = "KEY_S"
61463 = "KEY_T"
61464 = "KEY_U"
61465 = "KEY_V"
61466 = "KEY_W"
61467 = "KEY_X"
61468 = "KEY_Y"
61469 = "KEY_Z"
61470 = "KEY_1"
61471 = "KEY_2"
61472 = "KEY_3"
61473 = "KEY_4"
61474 = "KEY_5"
61475 = "KEY_6"
61476 = "KEY_7"
61477 = "KEY_8"
61478 = "KEY_9"
61479 = "KEY_0"
61480 = "KEY_ENTER"
61481 = "KEY_ESC"
61482 = "KEY_BACKSPACE"
61483 = "KEY_TAB"
61484 = "KEY_SPACE"
61485 = "KEY_MINUS"
61486 = "KEY_EQUAL"
61487 = "KEY_LEFT_BRACE"
61488 = "KEY_RIGHT_BRACE"
61489 = "KEY_BACKSLASH"
61490 = "KEY_NON_US_NUM"
61491 = "KEY_SEMICOLON"
61492 = "KEY_QUOTE"
61493 = "KEY_TILDE"
61494 = "KEY_COMMA"
61495 = "KEY_PERIOD"
61496 = "KEY_SLASH"
61497 = "KEY_CAPS_LOCK"
61498 = "KEY_F1"
61499 = "KEY_F2"
61500 = "KEY_F3"
61501 = "KEY_F4"
61502 = "KEY_F5"
61503 = "KEY_F6"
61504 = "KEY_F7"
61505 = "KEY_F8"
61506 = "KEY_F9"
61507 = "KEY_F10"
61508 = "KEY_F11"
61509 = "KEY_F12"
61510 = "KEY_PRINTSCREEN"
61511 = "KEY_SCROLL_LOCK"
61512 = "KEY_PAUSE"
61513 = "KEY_INSERT"
61514 = "KEY_HOME"
61515 = "KEY_PAGE_UP"
61516 = "KEY_DELETE"
61517 = "KEY_END"
61518 = "KEY_PAGE_DOWN"
61519 = "KEY_RIGHT"
61520 = "KEY_LEFT"
61521 = "KEY_DOWN"
61522 = "KEY_UP"
61523 = "KEY_NUM_LOCK"
61524 = "KEYPAD_SLASH"
61525 = "KEYPAD_ASTERIX"
61526 = "KEYPAD_MINUS"
61527 = "KEYPAD_PLUS"
61528 = "KEYPAD_ENTER"
61529 = "KEYPAD_1"
61530 = "KEYPAD_2"
61531 = "KEYPAD_3"
61532 = "KEYPAD_4"
61533 = "KEYPAD_5"
61534 = "KEYPAD_6"
61535 = "KEYPAD_7"
61536 = "KEYPAD_8"
61537 = "KEYPAD_9"
61538 = "KEYPAD_0"
61539 = "KEYPAD_PERIOD"
61540 = "KEY_NON_US_BS"
61541 = "KEY_MENU"
61544 = "KEY_F13"
61545 = "KEY_F14"
61546 = "KEY_F15"
61547 = "KEY_F16"
61548 = "KEY_F17"
61549 = "KEY_F18"
61550 = "KEY_F19"
61551 = "KEY_F20"
61552 = "KEY_F21"
61553 = "KEY_F22"
61554 = "KEY_F23"
61555 = "KEY_F24"
```

## Examples

Setting button 15 to key F on the first profile:

```sh
azeron-cli set-button 0 15 61449
```

Setting button 10 to Ctrl+P on the second profile:

```sh
azeron-cli set-button -m CTRL 1 10 61459
```
