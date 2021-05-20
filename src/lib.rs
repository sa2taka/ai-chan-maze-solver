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

pub fn solve(image_view: &mut DynamicImage) {
  let (width, height) = image_view.dimensions();
  let org_image = &mut DynamicImage::new_rgb16(width, height);
  let res = org_image.copy_from(image_view, 0, 0);
  let maze = parser::parse(image_view);
  let solved = maze_solver::solve(&maze);
  if solved.is_some() {
    solve_renderer::render(org_image, &maze, solved.unwrap());
    let res2 = org_image.save("solved1.png");
    match (res2) {
      Ok(_) => println!("done"),
      Err(err) => println!("{}", err),
    }
  }
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
