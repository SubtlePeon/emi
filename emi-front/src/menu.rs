use eframe::egui::{self, Button, Context};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub enum MenuResponse {
    None,
    CreateGoGame { game: emi_go::Game },
}

#[derive(Debug, Clone)]
pub enum Menu {
    Main {},
    Go { board_size: u32, ko_type: KoType },
}

#[derive(Debug, Clone, Copy)]
pub enum KoType {
    SimpleKo,
}

impl Menu {
    #[must_use]
    /// Currently just a test method
    pub fn draw(&mut self, ctx: &Context) -> MenuResponse {
        match self {
            Self::Main {} => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.button("Create Go Game").clicked() {
                        *self = Self::Go {
                            board_size: 19,
                            ko_type: KoType::SimpleKo,
                        };
                    }
                });
            }
            Self::Go { .. } => {
                if self.draw_menu_go(ctx) {
                    return MenuResponse::CreateGoGame {
                        game: self.create_go_game(),
                    };
                }
            }
        }
        MenuResponse::None
    }

    pub fn draw_menu_go(&mut self, ctx: &Context) -> bool {
        let Self::Go {
            board_size,
            ..
        } = self else { unreachable!() };

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("Create Go Game");

                ui.horizontal(|ui| {
                    ui.label("Go board size (drag): ");
                    ui.add(egui::widgets::DragValue::new(board_size).clamp_range(9..=19));
                });

                if ui.add(Button::new("Start")).clicked() {
                    info!("Starting go game!");
                    return true;
                }
                false
            })
            .inner
    }

    pub fn create_go_game(&self) -> emi_go::Game {
        use emi_go::Game;
        let Self::Go { board_size, .. } = self else {
            error!("Tried to create a go game while not in go menu; Creating size 19 game.");
            return Game::new(19);
        };
        // TODO: handle ko types
        Game::new(*board_size)
    }
}
