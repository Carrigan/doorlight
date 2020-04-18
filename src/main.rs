#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;

extern crate stm32l4xx_hal as hal;

extern crate cortex_m_rt as rt;
use crate::hal::prelude::*;
use crate::hal::delay::Delay;
use crate::rt::entry;

mod rainbow;
mod led;

enum Mode {
  Off,
  Busy,
  Party
}

#[inline(never)]
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();

    // Try a different clock configuration
    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);
    // let mut timer = Delay::new(cp.SYST, clocks);

    // Bit Banging output
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut clock = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut data = gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // Initialize our button and button state
    let button = gpioa.pa11.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    // LEDs
    let mut leds: [led::Color; 16] = [led::Color::black(); 16];
    led::refresh_display(&leds, &mut clock, &mut data);

    // For the program itself
    let mut was_pressed = false; // For checking that the press is a transition
    let mut mode = Mode::Off;
    let mut rainbow_state = rainbow::RainbowState::new(); // The rainbow state

    loop {
      // Check if the button is currently pressed
      let is_pressed = button.is_high().unwrap();

      // When these are in different states, a button press occurred
      if is_pressed && !was_pressed {
        mode = match mode {
          Mode::Off => Mode::Busy,
          Mode::Busy => Mode::Party,
          Mode::Party => Mode:: Off
        }
      }

      // Update the strip
      match mode {
        Mode::Off => led::change_strip_color(&mut leds, 0, 0, 0),
        Mode::Busy => led::change_strip_color(&mut leds, 255, 20, 20),
        Mode::Party => rainbow_state.advance_leds(&mut leds, 5)
      }

      led::refresh_display(&leds, &mut clock, &mut data);

      // Update if the button was prassed
      was_pressed = is_pressed;
    }
}
