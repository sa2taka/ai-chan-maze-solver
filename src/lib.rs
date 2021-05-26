extern crate image;

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};

mod libs {
  mod const_values;
  mod maze_cell;
  pub mod maze_solver;
  pub mod parser;
  mod point;
  pub mod solve_renderer;
}

use libs::maze_solver;
use libs::parser;
use libs::solve_renderer;

pub fn solve(image_view: &mut DynamicImage) -> Option<image::DynamicImage> {
  let (width, height) = image_view.dimensions();
  let mut org_image = DynamicImage::new_rgb16(width, height);
  let res = org_image.copy_from(image_view, 0, 0);
  let maze = parser::parse(image_view);
  let solved = maze_solver::solve(&maze);
  if solved.is_some() {
    solve_renderer::render(&mut org_image, &maze, solved.unwrap());
    return Some(org_image);
  }
  return None;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn solve_test() {
    let mut img = image::open("maze_1.png").unwrap();
    let result = solve(&mut img);
    if (result.is_some()) {
      result.unwrap().save("solve1.png");
    }

    let mut img = image::open("maze_4.png").unwrap();
    let result = solve(&mut img);
    if (result.is_some()) {
      result.unwrap().save("solve4.png");
    }
  }
}
