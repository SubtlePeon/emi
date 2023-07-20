#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use emi_go;

use crate::game_go::{state_go, GoResponse};
use crate::menu::{Menu, MenuResponse};

#[derive(Debug, Clone)]
pub enum Mode {
    /// A loading screen to be used in between modes.
    #[allow(dead_code)]
    Loading,
    /// The main menu. Not to be confused with any pause or pop-up
    /// menus.
    #[allow(dead_code)]
    Menu { menu: Menu },
    /// A shogi game where the user can make moves for both sides.
    /// No analysis.
    #[allow(dead_code)]
    ShogiPlay2,
    /// A go booard
    GoBoard { go_game: emi_go::Game },
}

#[derive(Debug, Clone)]
pub struct State {
    mode: Mode,
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: Mode::Menu {
                menu: Menu::Main {},
            },
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // General info
        match &mut self.mode {
            Mode::Menu { menu } => match menu.draw(ctx) {
                MenuResponse::None => {}
                MenuResponse::CreateGoGame { game } => {
                    self.mode = Mode::GoBoard { go_game: game };
                }
            },
            Mode::GoBoard { go_game } => {
                // Self::state_options_go(ctx);
                match state_go(ctx, go_game) {
                    GoResponse::None => {}
                    GoResponse::MainMenu => {
                        self.mode = Mode::Menu { menu: Menu::Main {} };
                        info!("going to main menu");
                    }
                }
            }
            Mode::Loading => todo!(),
            _ => unimplemented!(),
        }
    }
}

// May move to `thiserror`
#[allow(unused)]
#[derive(Debug)]
pub enum StateError {
    /// Used only for testing.
    TestError,
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::TestError => write!(f, "Testing error"),
        }
    }
}

impl std::error::Error for StateError {}
