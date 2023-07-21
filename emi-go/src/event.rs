use crate::Piece;

#[derive(Debug, Clone, PartialEq)]
/// An event in a go game.
///
/// Question: Should the `Capture` variant store all captured pieces to suppore
/// `.undo()`?
pub enum Event {
    Play {
        pos: [u32; 2],
        color: Piece,
        prev_ko: Option<[u32; 2]>,
    },
    Capture {
        pos: [u32; 2],
        color: Piece,
        captured: Vec<[u32; 2]>,
        prev_ko: Option<[u32; 2]>,
    },
    Edit {
        pos: [u32; 2],
        from: Piece,
        to: Piece,
    },
    Edits(Vec<Event>),
    Pass {
        color: Piece,
        prev_ko: Option<[u32; 2]>,
    },
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

    pub fn push(&mut self, evt: Event) {
        self.0.push(evt);
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.0.pop()
    }

    pub fn last_played_pos(&self) -> Option<[u32; 2]> {
        self.0.last()?.play_pos()
    }

    pub fn _last_played_color(&self) -> Option<Piece> {
        self.0.last()?.play_color()
    }

    pub fn last(&self) -> Option<&Event> {
        self.0.last()
    }

    pub fn last_was_pass(&self) -> bool {
        matches!(self.last(), Some(Pass { .. }))
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::new()
    }
}
