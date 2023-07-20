use eframe::egui::{self, Align, Button, Context, Layout, Pos2, Rect, Slider, Vec2};
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
                let one_quarter = ui.available_width() / 4.0;
                let one_half = ui.available_width() / 2.0;
                let height = ui.available_height();
                let mut ui = ui.child_ui(
                    Rect::from_min_max(
                        Pos2::new(one_quarter, 0.0),
                        Pos2::new(one_quarter + one_half, height),
                    ),
                    Layout::top_down(Align::Center),
                );
                ui.add_space(5.0);
                ui.label("Create Go Game");
                ui.add(
                    Slider::new(board_size, 9..=19)
                        .text("Board Size")
                        .clamp_to_range(true),
                );
                if ui
                    .add(Button::new("Start").min_size(Vec2::new(one_quarter, 0.0)))
                    .clicked()
                {
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
