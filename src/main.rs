#![no_std]
#![no_main]
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output},
    timer::timg::TimerGroup,
};
use esp_println::println;
esp_bootloader_esp_idf::esp_app_desc!(
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_NAME"),
    "00:00:00",
    "2024-01-01",
    "0.0.0",
    65536,
    0,
    65535,
    0
);


// TASK 1: The Independent Heartbeat Engine
#[embassy_executor::task]
async fn heartbeat_task(mut led_pin: Output<'static>) {
    loop {
        led_pin.set_high();
        Timer::after(Duration::from_millis(500)).await; // Non-blocking async sleep

        led_pin.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

// THE MASTER MAIN ENTRYPOINT
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    // 1. Initialize system clocks and peripheral registers safely
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // 2. Configure the asynchronous software timer groups using the new 0.23.1 drivers
    let mut timg0 = TimerGroup::new(peripherals.TIMG0);
    // Disable the Watchdog Timer to prevent reboot loops during development
    timg0.wdt.disable();
    esp_hal_embassy::init(timg0.timer0);

    // 3. Configure GPIO 4 as a digital output pin initialized low
    let led = Output::new(peripherals.GPIO4, Level::Low);

    println!("System Initialization Complete. Activating Async Tasks...");

    // 4. Spawn the heartbeat task onto the background executor runtime loop
    match spawner.spawn(heartbeat_task(led)) {
        Ok(_) => println!("Heartbeat task spawned successfully!"),
        Err(_) => println!("CRITICAL: Failed to spawn heartbeat task!"),
    }

    // MAIN EXECUTION LOOP (Task 2: System Monitor running concurrently)
    let mut loop_counter = 0;
    loop {
        println!("System Monitor: Loop Tick #{}", loop_counter);
        loop_counter += 1;

        // Wait 2 seconds before printing diagnostics again
        Timer::after(Duration::from_secs(2)).await;
    }
}