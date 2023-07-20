use crate::Piece;
use std::ops::{Index, IndexMut};

/// A glorified array of pieces
#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    side: u32,
    store: Vec<Piece>,
}

impl Board {
    /// Create a new board with side length `side`. The amount of intersections
    /// will be `side * side`.
    pub fn new(side: u32) -> Self {
        Self {
            side,
            store: vec![Piece::None; (side * side) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Piece {
        self.store[(y * self.side + x) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut Piece {
        &mut self.store[(y * self.side + x) as usize]
    }

    /// Gets the number of playable points on one side of the board.
    pub fn board_size(&self) -> u32 {
        self.side
    }

    /// Sets the coordinate to `Piece::None`.
    pub fn remove(&mut self, x: u32, y: u32) {
        *self.get_mut(x, y) = Piece::None;
    }

    /// Gets all valid coordinates surrounding the original coordinate
    /// provided.
    pub fn surround(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let ret = [
            (x.saturating_add(1), y),
            (x.saturating_sub(1), y),
            (x, y.saturating_add(1)),
            (x, y.saturating_sub(1)),
        ];
        ret.into_iter()
            .filter(|&(nx, ny)| self.coord_is_valid(nx, ny) && (nx, ny) != (x, y))
            .collect()
    }

    /// Returns true if the coordinate is on the board.
    pub fn coord_is_valid(&self, x: u32, y: u32) -> bool {
        x < self.side && y < self.side
    }

    /// Count the liberties of the group the stone at the specified
    /// coordinate is a part of.
    ///
    /// Returns `Option::None` if the specified coordinate is empty.
    pub fn liberties(&self, x: u32, y: u32) -> Option<u32> {
        // color of stones we're counting liberties of
        let color = self.get(x, y);
        if let Piece::None = color {
            return None;
        }
        let mut liberties = 0;

        let f = |x, y| {
            if let Piece::None = self.get(x, y) {
                liberties += 1;
                false
            } else if color == self.get(x, y) {
                true
            } else {
                false
            }
        };

        self.dfs((x, y), f);

        Some(liberties)
    }

    /// Count the number of stones in the group the stone at the
    /// specified position is a part of.
    ///
    /// Returns `Option::None` if the specified coordinate is empty.
    pub fn group_size(&self, init_x: u32, init_y: u32) -> Option<u32> {
        let color = self.get(init_x, init_y);
        if let Piece::None = color {
            return None;
        }
        let mut group_size = 0;

        let f = |x, y| {
            if self.get(x, y) == color {
                group_size += 1;
                true
            } else {
                false
            }
        };

        self.dfs((init_x, init_y), f);

        Some(group_size)
    }

    /// Returns the number of stones captured.
    pub fn capture(&mut self, init_x: u32, init_y: u32) -> Option<u32> {
        let mut marked = vec![];
        let color = self.get(init_x, init_y);
        if let Piece::None = color {
            return None;
        }
        let f = |x, y| {
            if self.get(x, y) == color {
                marked.push((x, y));
                true
            } else {
                false
            }
        };

        self.dfs((init_x, init_y), f);

        // Remove all captured stones
        for &(x, y) in marked.iter() {
            self[(x, y)] = Piece::None;
        }

        Some(marked.len() as u32)
    }

    /// Returns a `Vec` containing all the positions captured.
    /// Does not remove the stones that are to be captured.
    pub fn capture_(&self, init_x: u32, init_y: u32) -> Option<Vec<[u32; 2]>> {
        let mut marked = vec![];
        let color = self.get(init_x, init_y);
        if let Piece::None = color {
            return None;
        }
        let f = |x, y| {
            if self.get(x, y) == color {
                marked.push([x, y]);
                true
            } else {
                false
            }
        };

        self.dfs((init_x, init_y), f);

        Some(marked)
    }

    /// General Depth-First Search function. Takes a start position `pos`
    /// and a closure `f`. `f` takes in the next potentially searched
    /// position and returns `true` if the DFS should continue at that
    /// position.
    fn dfs(&self, pos: (u32, u32), mut f: impl FnMut(u32, u32) -> bool) {
        if !f(pos.0, pos.1) {
            return;
        }
        let mut stack = vec![pos];
        let mut visited = vec![];
        while let Some(next @ (x, y)) = stack.pop() {
            visited.push(next);
            for neighbor @ (nx, ny) in self.surround(x, y) {
                if visited.contains(&neighbor) {
                    continue;
                }
                if f(nx, ny) {
                    stack.push((nx, ny));
                }
            }
        }
    }
}

impl Index<(u32, u32)> for Board {
    type Output = Piece;
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self.store[(y * self.side + x) as usize]
    }
}

impl IndexMut<(u32, u32)> for Board {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        self.get_mut(x, y)
    }
}
