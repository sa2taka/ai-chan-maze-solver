use crate::libs::const_values::THRESHOLD;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use super::const_values::{FIREST_POINT, LEFT_CORNER, LEFT_CORNER_LIMIT, WALL_THICKNESS};
use super::point::Point;

/// 藍ちゃんの生成した迷路画像から迷路をパースする
pub fn parse(img: &mut DynamicImage) {
  binary(img);
  let diviced_cells = divide(img);
}

fn divide(img: &mut DynamicImage) -> Vec<DynamicImage> {
  let wall_length = measure_wall_length(img);
  let road_length = (wall_length * 3) - 1;
  let (width, _) = img.dimensions();

  let left_corner = Point {
    x: LEFT_CORNER,
    y: LEFT_CORNER,
  };
  let maze_edge = (width - left_corner.x * 2) as f32;
  let edge = (wall_length * 2 + wall_length + road_length) as f32;

  let size = (maze_edge / edge).round();

  let adjust = (maze_edge - edge * size) / size;
  let adjusted_edge = edge + adjust;

  let mut divided_cells: Vec<DynamicImage> = Vec::new();
  for y in 0..size.floor() as u32 {
    for x in 0..size.floor() as u32 {
      // u32を利用する関係で(x - 1)が-となってしまうため下記の計算式となっている
      let cell = img.crop(
        (left_corner.x as f32 + (x as f32 * adjusted_edge)).floor() as u32,
        (left_corner.y as f32 + (y as f32 * adjusted_edge)).floor() as u32,
        adjusted_edge.floor() as u32,
        adjusted_edge.floor() as u32,
      );
      divided_cells.push(cell);
    }
  }
  return divided_cells;
}

fn clip_maze(img: &mut DynamicImage, wall_length: u32) -> DynamicImage {
  let left_to_right = find_fist_wall_point(&img, Direction::LeftToRight).unwrap();
  let top_to_bottom = find_fist_wall_point(&img, Direction::TopToBottom).unwrap();

  let (width, height) = img.dimensions();
  let left_corner = Point {
    x: left_to_right.x,
    y: top_to_bottom.y,
  };

  // マージンのサイズが壁のサイズと同じであることを利用する
  let margin = wall_length / 2;

  return img.crop(
    left_corner.x - margin,
    left_corner.y - margin,
    width - left_corner.x * 2 + margin * 2,
    height - left_corner.y * 2 + margin * 2,
  );
}

fn find_fist_wall_point(img: &DynamicImage, direction: Direction) -> Option<Point> {
  for start in FIREST_POINT..LEFT_CORNER_LIMIT {
    let mut prev_color_change_location: u32 = FIREST_POINT;
    let mut prev_color: Rgba<u8> = if direction == Direction::LeftToRight {
      img.get_pixel(FIREST_POINT, start)
    } else {
      img.get_pixel(start, FIREST_POINT)
    };
    let first_color = prev_color.clone();

    for to in FIREST_POINT + 1..LEFT_CORNER_LIMIT {
      let color: Rgba<u8> = if direction == Direction::LeftToRight {
        img.get_pixel(to, start)
      } else {
        img.get_pixel(start, to)
      };

      let is_changed_color = calculate_color_distance(&color, &prev_color) > THRESHOLD as f32;

      // 壁を見つけた場合
      if is_changed_color
        && calculate_color_distance(&color, &first_color) < THRESHOLD as f32
        && (to - prev_color_change_location) < WALL_THICKNESS
      {
        return if direction == Direction::LeftToRight {
          Some(Point {
            x: prev_color_change_location,
            y: start,
          })
        } else {
          Some(Point {
            x: start,
            y: prev_color_change_location,
          })
        };
      }

      if is_changed_color {
        prev_color_change_location = to;
        prev_color = color;
      }
    }
  }
  return None;
}

fn measure_wall_length(img: &DynamicImage) -> u32 {
  let start = find_fist_wall_point(&img, Direction::LeftToRight).unwrap();
  let (width, _) = img.dimensions();
  let wall_color = img.get_pixel(start.x, start.y);
  for x in (start.x + 1)..width {
    let color = img.get_pixel(x, start.y);
    if calculate_color_distance(&color, &wall_color) > THRESHOLD as f32 {
      return x - start.x;
    }
  }
  return 0u32;
}

fn calculate_color_distance(color1: &Rgba<u8>, color2: &Rgba<u8>) -> f32 {
  let red = color1[0] as f32 - color2[0] as f32;
  let green = color1[1] as f32 - color2[1] as f32;
  let blue = color1[2] as f32 - color2[2] as f32;

  return (red.powi(2) + green.powi(2) + blue.powi(2)).sqrt();
}

fn binary(img: &mut DynamicImage) {
  // このあたりの数字の根拠は実ソースより引っ張ってきた
  // https://github.com/syuilo/ai/blob/master/src/modules/maze/themes.ts
  let (width, height) = img.dimensions();
  let left_top_corner_color: Rgba<u8> = img.get_pixel(0, 0);
  let base_binary = calc_binary_color(left_top_corner_color);

  let threshold = if base_binary < 40f32 {
    46f32
  } else {
    base_binary - 40f32
  };

  for y in 0..height {
    for x in 0..width {
      let pixel = img.get_pixel(x, y);
      let binary = calc_binary_color(pixel);

      let new_pixel = if binary > threshold {
        Rgba([255, 255, 255, 255])
      } else {
        Rgba([0, 0, 0, 255])
      };
      img.put_pixel(x, y, new_pixel);
    }
  }
}

fn calc_binary_color(color: Rgba<u8>) -> f32 {
  let red = color[0];
  let green = color[1];
  let blue = color[2];

  return 0.3 * red as f32 + 0.59 * green as f32 + 0.11 * blue as f32;
}

#[derive(PartialEq)]
enum Direction {
  LeftToRight,
  TopToBottom,
}
