#[derive(Default)]
pub struct Maze {
  pub width: u32,
  pub height: u32,
  pub cells: Vec<CellType>,
}

#[derive(Default)]
pub struct Way {
  pub top: bool,
  pub right: bool,
  pub bottom: bool,
  pub left: bool,
}
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CellType {
  Empty,
  Left,
  Right,
  Top,
  Bottom,
  LeftTop,
  LeftBottom,
  RightTop,
  RightBottom,
  LeftRightTop,
  LeftRightBottom,
  LeftTopBottom,
  RightTopBottom,
  LeftRight,
  TopBottom,
  Cross,
  Void,
}

impl CellType {
  pub fn get_way(&self) -> Way {
    return match self {
      Self::Empty => Way {
        top: false,
        right: false,
        bottom: false,
        left: false,
      },
      Self::Left => Way {
        top: false,
        right: false,
        bottom: false,
        left: true,
      },
      Self::Right => Way {
        top: false,
        right: true,
        bottom: false,
        left: false,
      },
      Self::Top => Way {
        top: true,
        right: false,
        bottom: false,
        left: false,
      },
      Self::Bottom => Way {
        top: false,
        right: false,
        bottom: true,
        left: false,
      },
      Self::LeftTop => Way {
        top: true,
        right: false,
        bottom: false,
        left: true,
      },
      Self::LeftBottom => Way {
        top: false,
        right: false,
        bottom: true,
        left: true,
      },
      Self::RightTop => Way {
        top: true,
        right: true,
        bottom: false,
        left: false,
      },
      Self::RightBottom => Way {
        top: false,
        right: true,
        bottom: true,
        left: false,
      },
      Self::LeftRightTop => Way {
        top: true,
        right: true,
        bottom: false,
        left: true,
      },
      Self::LeftRightBottom => Way {
        top: false,
        right: true,
        bottom: true,
        left: true,
      },
      Self::LeftTopBottom => Way {
        top: true,
        right: false,
        bottom: true,
        left: true,
      },
      Self::RightTopBottom => Way {
        top: true,
        right: true,
        bottom: true,
        left: false,
      },
      Self::LeftRight => Way {
        top: false,
        right: true,
        bottom: false,
        left: true,
      },
      Self::TopBottom => Way {
        top: true,
        right: false,
        bottom: true,
        left: false,
      },
      Self::Cross => Way {
        top: true,
        right: true,
        bottom: true,
        left: true,
      },
      Self::Void => Way {
        top: false,
        right: false,
        bottom: false,
        left: false,
      },
    };
  }
}
