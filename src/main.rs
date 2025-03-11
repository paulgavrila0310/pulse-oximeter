// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message through `defmt`. 

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
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let _p = embassy_rp::init(Default::default());

    info!("Hello world!");
    
    let delay = Duration::from_secs(1);
    loop {
        Timer::after(delay).await;
    }
}
