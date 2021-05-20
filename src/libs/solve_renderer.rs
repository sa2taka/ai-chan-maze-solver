extern crate image;
use super::const_values::LEFT_CORNER;
use super::maze_cell::Maze;
use super::point::Point;
use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Rgba;

pub fn render(img: &mut DynamicImage, maze: &Maze, solved: Vec<u32>) {
  let (img_width, img_height) = img.dimensions();
  let width = maze.width;
  let height = maze.height;
  let maze_img_size = img_width - LEFT_CORNER * 2;
  let img_cell_size = maze_img_size as f32 / width as f32;

  for index in solved {
    let y = index / height;
    let x = index % height;

    let point = Point {
      x: (x as f32 * img_cell_size) as u32 + LEFT_CORNER + (img_cell_size / 2f32) as u32,
      y: (y as f32 * img_cell_size) as u32 + LEFT_CORNER + (img_cell_size / 2f32) as u32,
    };

    render_point(img, point);
  }
}

fn render_point(img: &mut DynamicImage, point: Point) {
  let draw_color = [255, 0, 0, 255];
  let pixel: Rgba<u8> = Rgba(draw_color);
  let size = 12;

  for y in 0..size {
    for x in 0..size {
      img.put_pixel(point.x + x - size / 2, point.y + y - size / 2, pixel);
    }
  }
}
