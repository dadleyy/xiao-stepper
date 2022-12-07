#![no_main]
#![no_std]

extern crate panic_halt;

use hal::{clock::GenericClockController, delay::Delay, prelude::*};
use pac::{CorePeripherals, Peripherals};

use bsp::{entry, hal, pac, Led0};
use xiao_m0 as bsp;

#[entry]
fn main() -> ! {
  let mut peripherals = Peripherals::take().unwrap();
  let core = CorePeripherals::take().unwrap();
  let mut clocks = GenericClockController::with_external_32kosc(
    peripherals.GCLK,
    &mut peripherals.PM,
    &mut peripherals.SYSCTRL,
    &mut peripherals.NVMCTRL,
  );
  let pins = bsp::Pins::new(peripherals.PORT);

  let mut delay = Delay::new(core.SYST, &mut clocks);
  let mut led0: Led0 = pins.led0.into_push_pull_output();

  let mut step_pin = pins.a2.into_push_pull_output();
  let mut dir_pin = pins.a1.into_push_pull_output();
  let mut en_pin = pins.a0.into_push_pull_output();

  en_pin.set_low().expect("");
  dir_pin.set_high().expect("");

  loop {
    delay.delay_ms(1000u32);
    led0.toggle().unwrap();

    for _ in 0..(200 * 12) {
      step_pin.set_high().expect("");
      delay.delay_ms(1u32);
      step_pin.set_low().expect("");
    }

    dir_pin.set_low().expect("");
    delay.delay_ms(1000u32);

    for _ in 0..(200 * 12) {
      step_pin.set_high().expect("");
      delay.delay_ms(1u32);
      step_pin.set_low().expect("");
    }

    dir_pin.set_high().expect("");
  }
}
