use super::maze_cell::{CellType, Maze};
use core::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn solve(maze: &Maze) -> Option<Vec<u32>> {
  let mut heap: BinaryHeap<Entity> = BinaryHeap::new();
  let last_index = maze.width * maze.height - 1;
  let width = maze.width;
  let heigth = maze.height;

  let first = Entity {
    history: vec![0],
    prev_direction: Direction::Start,
  };
  heap.push(first);

  loop {
    let entity = heap.pop().unwrap();
    let history = entity.history;
    let index = *history.last().unwrap();
    let prev_direction = entity.prev_direction;

    if index == last_index {
      return Some(history);
    }

    let cell = maze.cells[index as usize];
    let way = cell.get_way();

    if way.top
      && prev_direction != Direction::Bottom
      && (cell != CellType::Cross || prev_direction == Direction::Top)
    {
      let next_index = index - width;
      // assert!(next_index > 0, "to_top: next_index is minus");
      let mut next_history = history.clone();
      next_history.push(next_index);

      let next_entity = Entity {
        history: next_history,
        prev_direction: Direction::Top,
      };

      heap.push(next_entity);
    }

    if way.left
      && prev_direction != Direction::Right
      && (cell != CellType::Cross || prev_direction == Direction::Left)
    {
      let next_index = index - 1;
      // assert!(
      //   next_index % width == width - 1,
      //   "to_left: next_index is next line"
      // );
      let mut next_history = history.clone();
      next_history.push(next_index);

      let next_entity = Entity {
        history: next_history,
        prev_direction: Direction::Left,
      };

      heap.push(next_entity);
    }

    if way.right
      && prev_direction != Direction::Left
      && (cell != CellType::Cross || prev_direction == Direction::Right)
    {
      let next_index = index + 1;
      // assert!(next_index % width == 0, "to_right: next_index is next line");

      let mut next_history = history.clone();
      next_history.push(next_index);

      let next_entity = Entity {
        history: next_history,
        prev_direction: Direction::Right,
      };

      heap.push(next_entity);
    }

    if way.bottom
      && prev_direction != Direction::Top
      && (cell != CellType::Cross || prev_direction == Direction::Bottom)
    {
      let next_index = index + width;
      // assert!(
      //   next_index <= last_index,
      //   "to_bottom: next_index is out of bounds"
      // );

      let mut next_history = history.clone();
      next_history.push(next_index);

      let next_entity = Entity {
        history: next_history,
        prev_direction: Direction::Bottom,
      };

      heap.push(next_entity);
    }
  }
  return None;
}

#[derive(Clone, PartialEq, Eq)]
struct Entity {
  history: Vec<u32>,
  prev_direction: Direction,
}

impl PartialOrd for Entity {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Entity {
  fn cmp(&self, other: &Self) -> Ordering {
    return other.history.len().cmp(&self.history.len());
  }
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
  Top,
  Right,
  Left,
  Bottom,
  Start,
}
