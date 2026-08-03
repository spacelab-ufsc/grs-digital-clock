#![no_std]
#![no_main]

#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types"
)]
#![deny(clippy::large_stack_frames)]

use defmt::error;
use embassy_executor::Spawner;
use esp_hal::clock::CpuClock;


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("{}", info);
    // TODO possibly blink all 7seg ?
    loop {}
}


// Required by ESP-IDF bootloader format
esp_bootloader_esp_idf::esp_app_desc!();


#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());

    let peripherals = esp_hal::init(config);

    // Initialize defmt output
    rtt_target::rtt_init_defmt!();

    // TODO beautiful splash text
    defmt::info!("ESP32-C3 firmware starting");


    //
    // Hardware initialization will happen here later:
    //
    // - I2C bus
    //      - AHT25
    //      - DS3231
    //
    // - SPI bus
    //      - MAX7219
    //
    // - Wi-Fi
    //      - initialized only when required
    //


    let _ = peripherals;


    loop {
        embassy_time::Timer::after_secs(60).await;
    }
}