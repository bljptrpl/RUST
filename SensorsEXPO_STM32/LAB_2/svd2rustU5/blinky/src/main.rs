#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32u575_pac as pac;
use cortex_m::peripheral::Peripherals;



#[entry]
fn main() -> ! {
let dp = pac::Peripherals::take().unwrap();
let mut cp = Peripherals::take().unwrap();

let rcc = dp.rcc;
// ahb2enr1: AHB2 peripheral clock enable register 1
// bit1: GPIOBEN: IO port B clock enable
// bit2: GPIOCEN: IO port C clock enable
// bit6: GPIOGEN: IO port G clock enable
rcc.rcc_ahb2enr1().modify(|_, w| {
    w.gpioben().set_bit() // enable GPIOB (bit 1)
    .gpiocen().set_bit() // enable GPIOC (bit 2)
    .gpiogen().set_bit() // enable GPIOG (bit 6)
    });
// Get the gpio peripherals needed to blink LEDs
let gpiob = dp.gpiob;
let gpioc = dp.gpioc;
let gpiog = dp.gpiog;

unsafe {
    // GPIO MODER: GPIO port mode register
    // GPIO modeX: Port x configuration bits (y = 0..15)
    // 00: Input mode (reset state)
    // 01: General purpose output mode
    // 10: Alternate function mode
    // 11: Analog mode
    // Set PB7 to output
    gpiob.gpio_moder().modify(|_, w| w.mode7().bits(0b01));
    // Set PC7 to output
    gpioc.gpio_moder().modify(|_, w| w.mode7().bits(0b01));
    // Set PG2 to output
    gpiog.gpio_moder().modify(|_, w| w.mode2().bits(0b01));
    }
    loop {
        gpioc.gpio_odr().modify(|_, w| w.od7().set_bit());
        delay(&mut cp.SYST, 160_000);
        gpioc.gpio_odr().modify(|_, w| w.od7().clear_bit());
        delay(&mut cp.SYST, 160_000);
        }
}
fn delay(syst: &mut cortex_m::peripheral::SYST, ticks: u32) {
    syst.set_reload(ticks);
    syst.clear_current();
    syst.enable_counter();
    while !syst.has_wrapped() {}
    }