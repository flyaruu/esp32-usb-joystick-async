## ESP32S3 joystick device

This repo is meant as a starting point to create your own joystick-like device.

It is based on splicing an esp-rs example for USB serial:

https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/embassy_usb_serial.rs

... with an embassy.rs example of doing creating a UDB-HID keyboard:

https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/usb_hid_keyboard.rs

Adding a joystick configuration to isn't too pretty (it isn't supported as well as keyboard devices), but there is an open (at the time of writing) issue that does explain how to do it, and it worked without issue, so I copied it:

https://github.com/twitchyliquid64/usbd-hid/issues/61#issuecomment-1826177494

I've put that code in the joystick.rs mod.

As usual, I'm using a channel and an embassy task that listens to that channel.

To feed that channel I've created an example (demo.rs) that simulates a rotating motion (using some sin/cos) as if you would be rotating
a joystick. 