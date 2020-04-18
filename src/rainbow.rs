use super::DotStar;

pub struct RainbowState {
  index: u8
}

// Between 0 - 252
// multiply by 6

fn index_to_color(led: &mut DotStar, index: u8) {
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

  pub fn advance_leds(&mut self, leds: &mut [DotStar], amount: u8) {
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
