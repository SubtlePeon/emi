#[allow(unused_imports)]
use tracing::{debug, info, trace};

mod board;
mod event;

use board::Board;
use event::{Event, Events};

/// A piece of either player's color. Or no piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    None,
    Black,
    White,
}

impl Piece {
    pub fn opposing(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Move {
    Place { pos: [u32; 2], color: Piece },
    Pass,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    /// Storage of current board state.
    board: Board,
    /// The color of the next player to move. Should not be `Piece::None`
    turn: Piece,
    /// Information about ko.
    ///
    /// Will be changed to support other variants of ko.
    ko_coord: Option<[u32; 2]>,
    events: Events,
}

impl Game {
    pub fn new(side: u32) -> Self {
        Self {
            board: Board::new(side),
            turn: Piece::Black,
            ko_coord: None,
            events: Events::new(),
        }
    }

    /// Reference `self.board`, which can be indexed with `board[(x, y)]`
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn apply_event_unchecked(&mut self, event: &Event) {
        use Event::*;

        let board = &mut self.board;
        match event {
            Play { pos: [x, y], color } => {
                *board.get_mut(*x, *y) = *color;
                self.ko_coord = None;
                self.next_turn();
            }
            Capture {
                pos: [x, y],
                color,
                captured,
            } => {
                *board.get_mut(*x, *y) = *color;
                for [rx, ry] in captured {
                    board.remove(*rx, *ry);
                }
                if captured.len() == 1 {
                    self.ko_coord = Some(*captured.last().unwrap());
                }
                self.next_turn();
            }
            Edit {
                pos: [x, y],
                from: _,
                to,
            } => {
                *board.get_mut(*x, *y) = *to;
            }
            Edits(edits) => {
                for edit in edits {
                    self.apply_event_unchecked(edit);
                }
            }
            Pass { .. } => self.next_turn(),
        }
    }

    pub fn construct_event(&self, move_: Move) -> Event {
        match move_ {
            Move::Pass => Event::Pass {
                color: self.next_to_play(),
            },
            Move::Place { pos: [x, y], color } => {
                // Check for capture
                let captures: Vec<_> = self
                    .board
                    .surround(x, y)
                    .into_iter()
                    .map(|(nx, ny)| {
                        if self.turn.opposing() != self.board.get(nx, ny) {
                            vec![]
                        } else if matches!(self.board.liberties(nx, ny), Some(0)) {
                            self.board.capture_(nx, ny).unwrap()
                        } else {
                            vec![]
                        }
                    })
                    .flatten()
                    .collect();

                if captures.len() > 0 {
                    Event::Capture {
                        pos: [x, y],
                        color,
                        captured: captures,
                    }
                } else {
                    Event::Play { pos: [x, y], color }
                }
            }
        }
    }

    /// Play a move
    pub fn play_(&mut self, move_: Move) -> Result<(), GoError> {
        if let Move::Place { pos: [x, y], color } = move_ {
            if self.board.get(x, y) != Piece::None {
                return Err(GoError::NotEmpty { move_ });
            }
            if self.turn != color {
                return Err(GoError::WrongTurn { move_ });
            }
            // Place piece
            self.board[(x, y)] = color;
        }

        let event = self.construct_event(move_);

        // Check for self capture
        if let Event::Play { pos: [x, y], .. } = event {
            if self.board.liberties(x, y) == Some(0) {
                self.board[(x, y)] = Piece::None;
                return Err(GoError::SelfCapture { move_ });
            }
        }

        // Remove the pre-played piece
        if let Move::Place { pos: [x, y], .. } = move_ {
            self.board[(x, y)] = Piece::None;
        }

        // Check for illegal ko capture
        if let Event::Capture { pos, captured, .. } = &event {
            if self.ko_coord == Some(*pos) && captured.len() == 1 {
                return Err(GoError::IllegalKo { move_ })
            }
        }

        self.apply_event_unchecked(&event);

        Ok(())
    }

    /// Play a move. (Old)
    pub fn play(&mut self, move_: Move) -> Result<(), GoError> {
        match move_ {
            Move::Place { pos: [x, y], color } => {
                // Check for valid placement
                if self.board.get(x, y) != Piece::None {
                    return Err(GoError::NotEmpty { move_ });
                }
                if self.turn != color {
                    return Err(GoError::WrongTurn { move_ });
                }
                // Place piece
                self.board[(x, y)] = color;
                // Check for capture
                let mut capturing = false;
                let mut captured_stones = 0;
                let mut capture_location = vec![];
                for (nx, ny) in self.board.surround(x, y) {
                    if let (Some(stones), Some(0)) =
                        (self.board.group_size(nx, ny), self.board.liberties(nx, ny))
                    {
                        if self.turn.opposing() != self.board.get(nx, ny) {
                            continue;
                        }
                        capturing = true;
                        captured_stones += stones;
                        capture_location.push([nx, ny]);
                    }
                }
                if capturing {
                    if captured_stones == 1 {
                        // Check for ko
                        if Some([x, y]) == self.ko_coord {
                            // Remove the stone we plaaced
                            self.board[(x, y)] = Piece::None;
                            return Err(GoError::IllegalKo { move_ });
                        } else {
                            self.ko_coord = capture_location.get(0).copied();
                        }
                    } else {
                        self.ko_coord = None;
                    }

                    // Remove captured stones
                    for [x, y] in capture_location {
                        self.board.capture(x, y);
                    }
                } else {
                    // Check for self-capture
                    if self
                        .board
                        .liberties(x, y)
                        .expect("Just placed a stone here")
                        == 0
                    {
                        // remove the stone we placed
                        self.board[(x, y)] = Piece::None;
                        return Err(GoError::SelfCapture { move_ });
                    } else {
                        self.ko_coord = None;
                    }
                }
                self.next_turn();
                Ok(())
            }
            Move::Pass => {
                self.next_turn();
                self.ko_coord = None;
                Ok(())
            }
        }
    }

    /// Changes turn.
    pub fn next_turn(&mut self) {
        self.turn = self.turn.opposing();
    }

    /// The piece color of the next player to play.
    pub fn next_to_play(&self) -> Piece {
        self.turn
    }

    /// The number of points across one side of the go board.
    ///
    /// Currently assumes the go board is square.
    pub fn board_size(&self) -> u32 {
        self.board.board_size()
    }
}

pub enum GoError {
    /// A move cannot be played because there is already a stone
    /// in that position.
    NotEmpty { move_: Move },
    /// The move made was for a player whose turn was not next.
    WrongTurn { move_: Move },
    /// Illegal self-capture
    SelfCapture { move_: Move },
    /// Illegal ko capture based on settings
    IllegalKo { move_: Move },
}
