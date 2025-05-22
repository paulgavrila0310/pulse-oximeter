// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with `defmt`.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Use the logging macros provided by defmt.
use defmt::*;

// Import interrupts definition module
mod irqs;

mod max30100;
use max30100::Max30100;

use embassy_rp::i2c::{Config, I2c};
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::I2C0;
use embassy_rp::i2c;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

const SOCK: usize = 4;
static RESOURCES: StaticCell<StackResources<SOCK>> = StaticCell::<StackResources<SOCK>>::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    // Init WiFi driver
    let (net_device, mut _control) = embassy_lab_utils::init_wifi!(&spawner, peripherals).await;

    // Default config for dynamic IP address
    let config = embassy_net::Config::dhcpv4(Default::default());

    // Init network stack
    let _stack = embassy_lab_utils::init_network_stack(&spawner, net_device, &RESOURCES, config);

    // info!("Hello world!");

    let i2c: I2c<'_, I2C0, i2c::Async> = I2c::new_async(
        peripherals.I2C0,
        peripherals.PIN_13,
        peripherals.PIN_12,
        Irqs,
        Config::default()
    );
    
    let mut max: Max30100<I2c<'_, I2C0, i2c::Async>> = Max30100::new(i2c);
    max.init().await.unwrap();
    
    loop {
        let (ir, red) = max.read_fifo().await.unwrap();
        defmt::info!("IR: {}, RED: {}", ir, red);
        Timer::after(Duration::from_millis(100)).await;
    }
    
}
