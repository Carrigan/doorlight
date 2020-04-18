#[derive(Clone, Copy)]
pub struct DotStar {
  pub red: u8,
  pub green: u8,
  pub blue: u8
}

impl DotStar {
  pub fn black() -> DotStar {
    DotStar {
      red: 0,
      green: 0,
      blue: 0
    }
  }
}
