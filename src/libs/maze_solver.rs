use crate::libs::maze_cell::Maze;

pub fn solve(maze: &Maze) {
  let last_index = maze.width * maze.height - 1;
}

#[derive(Clone)]
struct History {
  history: Vec<u32>,
  prev_direction: Direction,
}

#[derive(Clone)]
enum Direction {
  Top,
  Right,
  Left,
  Bottom,
  Start,
}
