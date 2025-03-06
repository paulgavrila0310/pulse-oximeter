// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app blinks the LED controlled through the WiFi chip and
//! logs the LED's state through the debugger using `defmt`.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
use defmt::*;

// Import interrupts definition module
mod irqs;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());

    // ********************* Initialize the WiFi driver ***********************
    // You can further use these variables if you implement WiFi applications.
    // For our blinky application, the intialization is needed to control the LED.
    let (_net_device, mut wifi_controller) = embassy_lab_utils::wifi::init_cy43w(p, &spawner).await;
    embassy_lab_utils::wifi::init_controller(
        &mut wifi_controller,
        cyw43::PowerManagementMode::PowerSave,
    )
    .await;
    // **************** End of initializing the WiFi driver *******************

    // The LED will toggle after 1 second.
    let delay = Duration::from_secs(1);
    loop {
        info!("led on!");
        wifi_controller.gpio_set(0, true).await;
        Timer::after(delay).await;

        info!("led off!");
        wifi_controller.gpio_set(0, false).await;
        Timer::after(delay).await;
    }
}
