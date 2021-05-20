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

  let mut prev_point = Point { x: 0, y: 0 };

  for index in solved {
    let y = index / height;
    let x = index % height;

    let point = Point {
      x: (x as f32 * img_cell_size) as u32 + LEFT_CORNER + (img_cell_size / 2f32) as u32,
      y: (y as f32 * img_cell_size) as u32 + LEFT_CORNER + (img_cell_size / 2f32) as u32,
    };

    render_point(img, point);

    if prev_point.x != 0 {
      render_line(img, point, prev_point);
    }
    prev_point = point;
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

fn render_line(img: &mut DynamicImage, from: Point, to: Point) {
  let draw_color = [255, 0, 0, 255];
  let pixel: Rgba<u8> = Rgba(draw_color);
  let size = 6;

  let x_dist = (from.x as i32 - to.x as i32).abs() as u32;
  let y_dist = (from.y as i32 - to.y as i32).abs() as u32;

  let range = if x_dist > y_dist {
    if from.x > to.x {
      to.x..from.x
    } else {
      from.x..to.x
    }
  } else {
    if from.y > to.y {
      to.y..from.y
    } else {
      from.y..to.y
    }
  };

  let fixed = if x_dist > y_dist { from.y } else { from.x };
  for j in range {
    for i in 0..size {
      if x_dist > y_dist {
        img.put_pixel(j, i + fixed - size / 2, pixel);
      } else {
        img.put_pixel(i + fixed - size / 2, j, pixel);
      };
    }
  }
}
