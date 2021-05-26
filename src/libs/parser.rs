use crate::libs::const_values::THRESHOLD;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use super::const_values::{
  FIREST_POINT, LEFT_CORNER, LEFT_CORNER_LIMIT, MAZE_AREA_SIZE, WALL_THICKNESS,
};
use super::maze_cell::{CellType, Maze};
use super::point::Point;

/// 藍ちゃんの生成した迷路画像から迷路をパースする
pub fn parse(img: &mut DynamicImage) -> Maze {
  binary(img);
  let wall_length = measure_wall_length(img);
  let mut diviced_cells = divide(img);

  let mut y = 0;
  for line in &diviced_cells {
    let mut x = 0;
    for cell in line {
      cell.save(format!("temp/{}_{}.png", y, x));
      x += 1;
    }
    y += 1;
  }

  let cells = parse_to_cells(&mut diviced_cells, wall_length);
  return Maze {
    width: diviced_cells.len() as u32,
    height: diviced_cells.len() as u32,
    cells: cells,
  };
}

fn divide(img: &mut DynamicImage) -> Vec<Vec<DynamicImage>> {
  let wall_length = measure_wall_length(img);
  let road_length = wall_length * 3;
  let (width, _) = img.dimensions();

  let left_corner = Point {
    x: LEFT_CORNER,
    y: LEFT_CORNER,
  };
  let maze_edge = (width - left_corner.x * 2) as f32;
  let edge = (wall_length * 2 + wall_length + road_length) as f32;

  let size = detect_size(edge) as f32;

  let adjust = (maze_edge - edge * size) / size;
  let adjusted_edge = edge + adjust;

  let mut divided_cells: Vec<Vec<DynamicImage>> = Vec::new();
  for y in 0..size as u32 {
    let mut divided_lines: Vec<DynamicImage> = Vec::new();
    for x in 0..size as u32 {
      // u32を利用する関係で(x - 1)が-となってしまうため下記の計算式となっている
      let cell = img.crop(
        (left_corner.x as f32 + (x as f32 * adjusted_edge)) as u32,
        (left_corner.y as f32 + (y as f32 * adjusted_edge)) as u32,
        adjusted_edge as u32,
        adjusted_edge as u32,
      );
      divided_lines.push(cell);
    }
    divided_cells.push(divided_lines);
  }
  return divided_cells;
}

fn parse_to_cells(cell_imgs: &mut Vec<Vec<DynamicImage>>, wall_length: u32) -> Vec<CellType> {
  let mut parsed_cells: Vec<CellType> = Vec::with_capacity(cell_imgs.len() * cell_imgs.len());
  for cells_line in cell_imgs {
    for cell in cells_line {
      parsed_cells.push(decide_cell(cell, wall_length));
    }
  }
  return parsed_cells;
}

fn decide_cell(cell_img: &mut DynamicImage, wall_length: u32) -> CellType {
  // 壁は1セルの中に井戸の「井」のように存在する可能性がある。
  // 左上から1, 2, 3, ... , 12 として、壁の存在によってセルを判定する
  // ただし、不必要な物は確認しない
  let (width, height) = cell_img.dimensions();
  let margin = wall_length / 2;

  let start_point1 = Point {
    x: margin / 2,
    y: margin / 2,
  };
  let start_point2 = Point {
    x: width / 2,
    y: margin / 2,
  };
  let start_point3 = Point {
    x: margin / 2,
    y: height / 2,
  };
  let start_point4 = Point {
    x: width / 2,
    y: height / 2,
  };

  let has_wall_1 = has_wall(cell_img, Direction::LeftToRight, &start_point1);
  let has_wall_3 = has_wall(cell_img, Direction::TopToBottom, &start_point1);
  let has_wall_4 = has_wall(cell_img, Direction::TopToBottom, &start_point2);
  let has_wall_6 = has_wall(cell_img, Direction::LeftToRight, &start_point3);
  let has_wall_7 = has_wall(cell_img, Direction::LeftToRight, &start_point4);
  let has_wall_9 = has_wall(cell_img, Direction::TopToBottom, &start_point4);

  // 基本的には4, 6, 7, 9("井"の中心の4枚の壁)でわかるが、交差しているパターンが確認できない
  // そのため、1, 3を活用する。

  // まず交差と直線を確認する
  //  TopBottom
  if !has_wall_4 && !has_wall_9 && has_wall_6 && has_wall_7 {
    // 左右の道のある壁がある場合
    if has_wall_3 {
      return CellType::Cross;
    } else {
      return CellType::TopBottom;
    }
  }

  //  LeftRight
  if has_wall_4 && has_wall_9 && !has_wall_6 && !has_wall_7 {
    // 上下の道のある壁がある場合
    if has_wall_1 {
      return CellType::Cross;
    } else {
      return CellType::LeftRight;
    }
  }

  // 残りは真ん中の4, 6, 7, 9のみで判断可能
  // 6: LEFT, 7: RIGHT, 4: TOP, 9: BOTTOM
  return match (has_wall_6, has_wall_7, has_wall_4, has_wall_9) {
    (false, false, false, false) => CellType::Empty,
    (false, false, false, true) => CellType::LeftRightTop,
    (false, false, true, false) => CellType::LeftRightBottom,
    (false, false, true, true) => CellType::LeftRight, // 上記でマッチ済み
    (false, true, false, false) => CellType::LeftTopBottom,
    (false, true, false, true) => CellType::LeftTop,
    (false, true, true, false) => CellType::LeftBottom,
    (false, true, true, true) => CellType::Left,
    (true, false, false, false) => CellType::RightTopBottom,
    (true, false, false, true) => CellType::RightTop,
    (true, false, true, false) => CellType::RightBottom,
    (true, false, true, true) => CellType::Right,
    (true, true, false, false) => CellType::TopBottom, // 上記でマッチ済み,
    (true, true, false, true) => CellType::Top,
    (true, true, true, false) => CellType::Bottom,
    (true, true, true, true) => CellType::Void,
  };
}

fn has_wall(img: &mut DynamicImage, direction: Direction, first_point: &Point) -> bool {
  let (width, height) = img.dimensions();
  let limit = if direction == Direction::LeftToRight {
    first_point.x + width / 2
  } else {
    first_point.y + height / 2
  };

  return find_first_wall_point_on_line(img, direction, first_point, limit).is_some();
}

fn find_first_wall_point(
  img: &DynamicImage,
  direction: Direction,
  first_point: &Point,
  limit: &Point,
) -> Option<Point> {
  let (first_start, first_to) = if direction == Direction::LeftToRight {
    (first_point.y, first_point.x)
  } else {
    (first_point.x, first_point.y)
  };

  let (start_limit, to_limit) = if direction == Direction::LeftToRight {
    (limit.y, limit.x)
  } else {
    (limit.x, limit.y)
  };

  for start in first_start..start_limit {
    let origin = if direction == Direction::LeftToRight {
      Point {
        x: first_to,
        y: start,
      }
    } else {
      Point {
        x: start,
        y: first_to,
      }
    };

    let result = find_first_wall_point_on_line(img, direction, &origin, to_limit);

    if result.is_some() {
      return result;
    }
  }
  return None;
}

fn find_first_wall_point_on_line(
  img: &DynamicImage,
  direction: Direction,
  origin: &Point,
  limit: u32,
) -> Option<Point> {
  let mut prev_color_change_location = if direction == Direction::LeftToRight {
    origin.x
  } else {
    origin.y
  };
  let mut prev_color: Rgba<u8> = img.get_pixel(origin.x, origin.y);
  let first_color = prev_color.clone();

  for to in prev_color_change_location + 1..limit {
    let color: Rgba<u8> = if direction == Direction::LeftToRight {
      img.get_pixel(to, origin.y)
    } else {
      img.get_pixel(origin.x, to)
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
          y: origin.y,
        })
      } else {
        Some(Point {
          x: origin.x,
          y: prev_color_change_location,
        })
      };
    }

    if is_changed_color {
      prev_color_change_location = to;
      prev_color = color;
    }
  }
  return None;
}

fn measure_wall_length(img: &DynamicImage) -> u32 {
  let fisrst_point = Point {
    x: FIREST_POINT,
    y: FIREST_POINT,
  };
  let limit_point = Point {
    x: LEFT_CORNER_LIMIT,
    y: LEFT_CORNER_LIMIT,
  };
  let start =
    find_first_wall_point(&img, Direction::LeftToRight, &fisrst_point, &limit_point).unwrap();
  let (width, _) = img.dimensions();
  let wall_color = img.get_pixel(start.x, start.y);
  for x in (start.x + 1)..width {
    let color = img.get_pixel(x, start.y);
    if calculate_color_distance(&color, &wall_color) > THRESHOLD as f32 {
      return x - start.x - 1;
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
    base_binary - 4f32
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

fn detect_size(edge_size: f32) -> u32 {
  let mut prev_diff = MAZE_AREA_SIZE as f32;
  for i in 3..100 {
    let diff = ((MAZE_AREA_SIZE as f32 / i as f32) - edge_size).abs();
    if diff > prev_diff {
      return i;
    }
    prev_diff = diff;
  }
  return 100;
}

#[derive(PartialEq, Copy, Clone)]
enum Direction {
  LeftToRight,
  TopToBottom,
}
