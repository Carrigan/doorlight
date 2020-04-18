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
      green: 0xFF,
      blue: 0xFF
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
    let mut timer = Delay::new(cp.SYST, clocks);

    // Bit Banging output
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut clock = gpioa.pa0.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut data = gpioa.pa1.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // Initialize our button and button state
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb2);
    let button = gpioa.pa11.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    // LEDs
    let mut leds: [DotStar; 16] = [DotStar::white(); 16];
    change_strip_color(&mut leds, 0x00, 0x00, 0xFF);
    refresh_display(&leds, &mut clock, &mut data);

    // For the program loop
    let mut was_pressed = false; // For checking that the press is a transition
    let mut is_blue = true;      // For checking what color we are on

    // loop {
    //   let is_pressed = button.is_high().unwrap();

    //   if is_pressed && !was_pressed {
    //     timer.delay_ms(100 as u32);


    //     if is_blue {
    //       change_strip_color(&mut leds, 0xFF, 0x00, 0x00);
    //     } else {
    //       change_strip_color(&mut leds, 0x00, 0x00, 0xFF);
    //     }

    //     is_blue = !is_blue;
    //     refresh_display(&leds, &mut clock, &mut data);
    //     timer.delay_ms(100 as u32);
    //   }

    //   was_pressed = is_pressed;
    // }

    let mut rainbow_state = rainbow::RainbowState::new();

    loop {
      rainbow_state.advance_leds(&mut leds, 5);
      refresh_display(&leds, &mut clock, &mut data);
      timer.delay_ms(10 as u32);
    }
}

fn change_strip_color(leds: &mut [DotStar], red: u8, green: u8, blue: u8) {
  for led in leds {
    led.blue = blue;
    led.green = green;
    led.red = red;
  }
}

fn send_byte<CP: hal::prelude::OutputPin, DP: hal::prelude::OutputPin>(byte: u8, clock: &mut CP, data: &mut DP) {
    for x in 0..8 {
        let current_bit = (1 & (byte >> (7 - x))) == 1;

        if current_bit { data.set_low(); } else { data.set_high(); }
        clock.set_low();
        clock.set_high();
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
