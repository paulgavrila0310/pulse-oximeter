//! This module contains functions for initializing and handling WiFi components.

use cyw43::{Control, NetDriver, PowerManagementMode};
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_rp::{
    gpio::{Level, Output},
    peripherals::{DMA_CH2, PIO0},
    pio::{InterruptHandler as PioInterruptHandler, Pio},
};
use static_cell::StaticCell;

embassy_rp::bind_interrupts!(struct PioIrq {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
});

/// This task runs the wifi chip driver. This will need to run in an infinite loop.
#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH2>>,
) -> ! {
    runner.run().await
}

/// Initialize the CYW43 Wifi chip driver and spawn the task that runs the driver.
///
/// # Example
/// ```rust,ignore
/// // Bind the PIO interrupt number and the interrupt handler.
/// bind_interrupts!(struct Irqs {
///     PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
/// });
///
/// // Initialize the RP235x peripherals.
/// let p = embassy_rp::init(Default::default());
/// let (_net_device, mut control, runner) = embassy_lab_utils::init_cy43w(p, Irqs, &spawner);
/// ```
///
/// Returns a handle to the network device, control handle and a runner for driving the low level
/// stack.
pub async fn init_cy43w(
    p: embassy_rp::Peripherals,
    spawner: &embassy_executor::Spawner,
) -> (NetDriver<'static>, Control<'static>) {
    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, PioIrq);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH2,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    (net_device, control)
}

/// Initialize the WiFi controller with the given power management mode.
///
/// # Example
/// ```rust,ignore
///
/// // Initialize the RP235x peripherals.
/// let p = embassy_rp::init(Default::default());
/// let (_net_device, mut control, runner) = embassy_lab_utils::init_cy43w(p, Irqs, &spawner);
/// embassy_lab_utils::init_wifi_controller(&mut control);
/// ```
pub async fn init_controller(controller: &mut Control<'static>, mode: PowerManagementMode) {
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");
    controller.init(clm).await;
    controller.set_power_management(mode).await;
}
