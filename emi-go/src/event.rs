use crate::{Board, Piece};

#[derive(Debug, Clone, PartialEq)]
/// An event in a go game.
///
/// Question: Should the `Capture` variant store all captured pieces to suppore
/// `.undo()`?
pub enum Event {
    Play {
        pos: [u32; 2],
        color: Piece,
    },
    Capture {
        pos: [u32; 2],
        color: Piece,
        captured: Vec<[u32; 2]>,
    },
    Edit {
        pos: [u32; 2],
        from: Piece,
        to: Piece,
    },
    Edits(Vec<Event>),
    Pass {
        color: Piece,
    }
}

use Event::*;

impl Event {
    pub fn play_pos(&self) -> Option<[u32; 2]> {
        match self {
            Play { pos, .. } => Some(*pos),
            Capture { pos, .. } => Some(*pos),
            _ => None,
        }
    }

    pub fn play_color(&self) -> Option<Piece> {
        match self {
            Play { color, .. } => Some(*color),
            Capture { color, .. } => Some(*color),
            _ => None,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Events(Vec<Event>);

impl Events {
    pub fn new() -> Self {
        Events(Vec::new())
    }

    pub fn last_played_pos(&self) -> Option<[u32; 2]> {
        self.0.last()?.play_pos()
    }

    pub fn last_played_color(&self) -> Option<Piece> {
        self.0.last()?.play_color()
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}
