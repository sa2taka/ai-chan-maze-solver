#[derive(Default)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({} {})", self.x, self.y)
  }
}
