#![no_std]
#![no_main]

use alloc::boxed::Box;
use embassy_executor::{task, Spawner};
use embassy_sync::{blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex}, signal::Signal};
use embassy_time::{Duration, Timer};
use embassy_usb::{class::hid::{HidReaderWriter, HidWriter, State}, Builder, UsbDevice};
use esp32_usb_joystick_async::{demo::Rotator, joystick::JoystickReport};
use esp_backtrace as _;
use esp_hal::{clock::CpuClock, otg_fs::{asynch::{Config, Driver}, Usb}};
use esp_hal_embassy::main;
use log::{info, warn};
use usbd_hid::descriptor::SerializedDescriptor;

extern crate alloc;

static JOYSTICK_SIGNAL: Signal<CriticalSectionRawMutex, JoystickReport> = Signal::new();

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);

    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);

    // Create the driver, from the HAL.
    let ep_out_buffer = Box::leak(Box::new([0u8; 1024]));
    let config = Config::default();

    let driver = Driver::new(usb, ep_out_buffer, config);


    let mut config = embassy_usb:: Config::new(0xa0de, 0xdafe);
    config.manufacturer = Some("Floodplain");
    config.product = Some("Frank's Joystick");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let config_descriptor = Box::leak(Box::new([0; 256]));
    let bos_descriptor = Box::leak(Box::new([0; 256]));
    // You can also add a Microsoft OS descriptor.
    let msos_descriptor = Box::leak(Box::new([0; 256]));
    let control_buf = Box::leak(Box::new([0; 64]));


    let mut builder = Builder::new(
        driver,
        config,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control_buf,
    );


    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: JoystickReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };

    let state = Box::leak(Box::new(State::new()));


    let mut writer: HidWriter<'_, Driver<'_>, 8> = HidWriter::new(&mut builder, state, config);

    let usb = builder.build();

    // let report = JoystickReport { x: 1, y: 2, rx: 0, ry: 0, buttons: 0 };
    // writer.write_serialize(&report).await.unwrap();

    spawner.must_spawn(run_usb(usb));
    spawner.must_spawn(run_demo());
    spawner.must_spawn(usb_writer(writer));
    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin
}

#[task]
async fn run_usb(mut usb: UsbDevice<'static,Driver<'static>>)->! {
    usb.run().await
}

#[task]
async fn run_demo() {
    let rotator = Rotator::default();
    for report in rotator {
        info!("Joystick report:  {:?}", report);
        JOYSTICK_SIGNAL.signal(report);
        Timer::after_millis(10).await;
    }
}

#[task]
async fn usb_writer(mut writer: HidWriter<'static, Driver<'static>, 8>)->! {
    loop {
        match writer.write_serialize(&JOYSTICK_SIGNAL.wait().await).await {
            Ok(_) => {},
            Err(e) => {
                warn!("Write failed: {:?}",e);
            },
        }
        Timer::after_millis(5).await;
    }
}