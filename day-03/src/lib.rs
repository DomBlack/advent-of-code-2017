/// A simple struct to track the position
#[derive(Hash, Eq, PartialEq, Default, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

/// A simple enum to track our current direction
pub enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP,
}

impl Default for Direction {
    fn default() -> Self { Direction::RIGHT }
}

impl Position {
    /// Creates a new position, moving around the spiral grid
    pub fn mov(&self, direction: &mut Direction, grid_size: &mut i32) -> Position {
        match *direction {
            Direction::LEFT  => {
                if self.x == -*grid_size {
                    *direction = Direction::DOWN;
                    self.down()
                } else {
                    self.left()
                }
            },
            Direction::RIGHT => {
                if self.x == *grid_size {
                    *direction = Direction::UP;
                    *grid_size = *grid_size + 1;
                    self.up()
                } else {
                    self.right()
                }
            },
            Direction::DOWN  => {
                if self.y == -*grid_size {
                    *direction = Direction::RIGHT;
                    self.right()
                } else {
                    self.down()
                }
            },
            Direction::UP    => {
                if self.y == *grid_size {
                    *direction = Direction::LEFT;
                    self.left()
                } else {
                    self.up()
                }
            },
        }
    }

    pub fn neighbours(&self) -> [Position; 8] {
        [
            self.left(),
            self.left().up(),
            self.up(),
            self.right().up(),
            self.right(),
            self.right().down(),
            self.down(),
            self.left().down()
        ]
    }

    fn left(&self) -> Position {
        Position { x: self.x - 1, y: self. y}
    }

    fn right(&self) -> Position {
        Position { x: self.x + 1, y: self. y}
    }

    fn down(&self) -> Position {
        Position { x: self.x, y: self. y - 1}
    }

    fn up(&self) -> Position {
        Position { x: self.x, y: self. y + 1}
    }
}