extern crate image;

use image::DynamicImage;

mod libs {
  mod const_values;
  mod maze_cell;
  pub mod maze_solver;
  pub mod parser;
  mod point;
}

use libs::maze_solver;
use libs::parser;

pub fn solve(image_view: &mut DynamicImage) {
  let maze = parser::parse(image_view);
  let solved = maze_solver::solve(&maze);
  println!("parse");
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
