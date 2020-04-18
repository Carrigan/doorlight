use super::led::Color;

pub struct RainbowState {
  index: u8
}

// There are 6 stages as follows, where / means "is ramping up", \ means "is
// ramping down", and * means is fully on.
//   0 1 2 3 4 5
// R * \     / *
// G / * * \
// B     / * * \
// Since 6 does not go into 256, wrap our index at 252 making 6 groups of 42.

fn index_to_color(led: &mut Color, index: u8) {
  let step = index / 42;
  let remainder = index % 42;

  led.red = match step {
    0 | 5 => 252,
    4 => remainder * 6,
    1 => 252 - (remainder * 6),
    _ => 0
  };

  led.green = match step {
    1 | 2 => 252,
    0 => remainder * 6,
    3 => 252 - (remainder * 6),
    _ => 0
  };

  led.blue = match step {
    3 | 4 => 252,
    2 => remainder * 6,
    5 => 252 - (remainder * 6),
    _ => 0
  };
}

// This applies an index shift between LEDs, where a cycle is 252 units.
fn rotate_position(index: u8, position: u8) -> u8 {
  let offset = position * 15;

  if index > (252 - offset) {
    offset - (252 - index)
  } else {
    index + offset
  }
}

impl RainbowState {
  pub fn new() -> RainbowState {
    RainbowState { index: 0 }
  }

  pub fn advance_leds(&mut self, leds: &mut [Color], amount: u8) {
    // Advance
    self.index += amount;
    if self.index >= 252 {
      self.index = 0;
    }

    // Offset them and convert them to colors
    for (position, mut led) in &mut leds.into_iter().enumerate() {
      let shifted_index = rotate_position(self.index, position as u8);
      index_to_color(&mut led, shifted_index);
    }
  }
}
