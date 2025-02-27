use usbd_hid::descriptor::gen_hid_descriptor;
use usbd_hid::descriptor::generator_prelude::*;

/// JoystickReport describes a report and its companion descriptor that can be
/// used to send joystick movements and button presses to a host.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = JOYSTICK) = {
        (collection = APPLICATION, usage = POINTER) = {
            (usage = X,) = {
                #[item_settings data,variable,absolute] x=input;
            };
            (usage = Y,) = {
                #[item_settings data,variable,absolute] y=input;
            };
            (usage = 0x33,) = {
                #[item_settings data,variable,absolute] rx=input;
            };
            (usage = 0x34,) = {
                #[item_settings data,variable,absolute] ry=input;
            };
        };
        (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
        }
    }
)]
#[allow(dead_code)]
pub struct JoystickReport {
    pub x: i8,
    pub y: i8,
    pub rx: i8,
    pub ry: i8,
    pub buttons: u8,
}