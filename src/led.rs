
#[derive(Clone, Copy)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8
}

impl Color {
  pub fn black() -> Color {
    Color {
      red: 0x00,
      green: 0x00,
      blue: 0x00
    }
  }
}

pub fn change_strip_color(leds: &mut [Color], red: u8, green: u8, blue: u8) {
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

pub fn refresh_display<CP: hal::prelude::OutputPin, DP: hal::prelude::OutputPin>(leds: &[Color], clock: &mut CP, data: &mut DP) {
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
