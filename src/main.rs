#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use panic_probe as _;

// Options for bind_interrupts are listed below:

// This fails as: cannot find type `InterruptHandler` in module `i2c`
bind_interrupts!(struct Irqs {
    I2C1 => i2c::InterruptHandler<peripherals::I2C1>;
});

// This fails as:
// cannot find type `I2C1_EV` in module `$crate::interrupt::typelevel`
// not found in `$crate::interrupt::typelevel` [E0412]
// and:
// cannot find type `I2C1_ER` in module `$crate::interrupt::typelevel`
// not found in `$crate::interrupt::typelevel` [E0412]

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(config);

    let i2c_config = i2c::Config::default();

    let mut i2c1 = i2c::I2c::new(
        p.I2C1,
        p.PA9,
        p.PA10,
        Irqs,
        NoDma,
        NoDma,
        Hertz(1_000_000),
        i2c_config,
    );

    loop {}
}
