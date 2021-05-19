extern crate image;

use image::DynamicImage;

mod libs {
  mod const_values;
  pub mod parser;
  mod point;
}

use libs::parser;

pub fn solve(image_view: &mut DynamicImage) {
  parser::parse(image_view);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn solve_test() {
    let mut img = image::open("maze_1.png").unwrap();
    solve(&mut img);
  }
}
