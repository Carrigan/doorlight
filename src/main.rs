#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_halt;

extern crate stm32l4xx_hal as hal;

#[macro_use]
extern crate cortex_m_rt as rt;
use crate::hal::prelude::*;
use crate::hal::delay::Delay;
use crate::rt::entry;


#[derive(Clone, Copy)]
pub struct DotStar {
  pub red: u8,
  pub green: u8,
  pub blue: u8
}

impl DotStar {
  pub fn white() -> DotStar {
    DotStar {
      red: 0xFF,
      green: 0,
      blue: 0
    }
  }
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

    // LEDs
    let leds: [DotStar; 16] = [DotStar::white(); 16];
    refresh_display(&leds, &mut clock, &mut data);

    loop {}
}

fn send_byte<CP: hal::prelude::OutputPin, DP: hal::prelude::OutputPin>(byte: u8, clock: &mut CP, data: &mut DP) {
    for x in 0..8 {
        let current_bit = (1 & (byte >> x)) == 1;

        if current_bit { data.set_high(); } else { data.set_low(); }
        clock.set_high();
        clock.set_low();
    }
}

fn refresh_display<CP: hal::prelude::OutputPin, DP: hal::prelude::OutputPin>(leds: &[DotStar], clock: &mut CP, data: &mut DP) {
    // Send the start frame
    for _ in 0..4 { send_byte(0, clock, data) }

    // For each LED, send a frame
    for led in leds {
        send_byte(0xFF, clock, data);
        send_byte(led.blue, clock, data);
        send_byte(led.green, clock, data);
        send_byte(led.red, clock, data);
    }

    // Send the end frame
    for _ in 0..4 { send_byte(0xFF, clock, data) }
}
