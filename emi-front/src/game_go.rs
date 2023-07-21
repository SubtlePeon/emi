use eframe::egui::{
    self, emath, vec2, Color32, Frame, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Vec2,
};
use emi_go::{Move, Piece};
use tracing::debug;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Mark {
    /// No mark.
    None,
    /// unimplemented
    Triangle,
    /// unimplemented
    Square,
    /// unimplemented
    Numbering,
    /// unimplemented
    Lettering,
    /// Draws a dot on the stone.
    Dot,
}

#[derive(Debug, Clone)]
pub enum GoResponse {
    None,
    MainMenu,
}

fn draw_mark(painter: &Painter, piece_color: Piece, pos: Pos2, piece_radius: f32, mark: Mark) {
    let color = match piece_color {
        Piece::None => {
            return;
        }
        Piece::Black => Color32::WHITE.gamma_multiply(0.9),
        Piece::White => Color32::BLACK.gamma_multiply(0.9),
    };

    match mark {
        Mark::None => {}
        Mark::Dot => _ = painter.add(Shape::circle_filled(pos, piece_radius / 3.0, color)),
        _ => unimplemented!(),
    }
}

fn draw_go_piece(painter: &Painter, piece_color: Piece, pos: Pos2, radius: f32, mark: Mark) {
    let color = match piece_color {
        Piece::None => {
            return;
        }
        Piece::Black => Color32::BLACK,
        Piece::White => Color32::WHITE,
    };
    let piece = Shape::circle_filled(pos, radius, color);
    painter.add(piece);
    draw_mark(painter, piece_color, pos, radius, mark);
}

fn draw_go_board(go_game: &emi_go::Game, painter: &egui::Painter, board_rect: Rect) {
    // number of points along one side of the board (usually 9, 13, or 19)
    let board_points = go_game.board_size() as f32;

    let board_size = board_rect.width();
    // The size of one square on the board
    let unit = board_size / board_points;
    let thickness = (board_size / 400.0).ceil() * 0.5;

    let board = Shape::rect_filled(
        board_rect,
        0.0,
        // #e7c88c
        Color32::from_rgb(0xe7, 0xc8, 0x8c),
    );

    painter.add(board);

    // Converts from game coordinate to screen coordinate
    let coord_fn = |[x, y]: [f32; 2]| {
        board_rect.left_top() + vec2(x * unit, y * unit) + Vec2::splat(unit / 2.0)
    };

    // draw lines
    for i in 0..board_points as usize {
        let horz_line = Shape::line_segment(
            [
                coord_fn([0.0, i as f32]),
                coord_fn([board_points - 1.0, i as f32]),
            ],
            Stroke::new(thickness, Color32::BLACK),
        );
        let vert_line = Shape::line_segment(
            [
                coord_fn([i as f32, 0.0]),
                coord_fn([i as f32, board_points - 1.0]),
            ],
            Stroke::new(thickness, Color32::BLACK),
        );
        painter.add(horz_line);
        painter.add(vert_line);
    }

    if board_points <= 4.0 {
        return;
    }
    // Distance from the edge of the board (intersections)
    // for dots to be placed
    // One quarter
    let a = if board_points < 17.0 {
        (board_points / 3.0).floor() - 1.0
    } else {
        (board_points / 4.0).floor() - 1.0
    };
    // Half
    let b = (board_points / 2.0).ceil() - 1.0;
    // Three quarters
    let c = board_points - a - 1.0;
    let mut dots = Vec::with_capacity(9);
    // Add corners
    dots.push([a, a]);
    dots.push([a, c]);
    dots.push([c, a]);
    dots.push([c, c]);

    if board_points as u32 % 2 == 1 {
        // center
        dots.push([b, b]);

        // Add sides
        if board_points >= 19.0 {
            dots.push([a, b]);
            dots.push([c, b]);
            dots.push([b, a]);
            dots.push([b, c]);
        }
    }

    // Draw the dots on the board
    for dot_coord in dots {
        painter.add(Shape::circle_filled(
            coord_fn(dot_coord),
            thickness * 3.0,
            Color32::BLACK,
        ));
    }
}

fn game_go_display_pieces(go_game: &emi_go::Game, painter: &egui::Painter, board_rect: Rect) {
    let board_size = board_rect.width();
    // The number of points in the board along one side.
    let board_points = go_game.board_size();
    let r = board_size / board_points as f32 / 2.0;
    for x in 0..board_points {
        for y in 0..board_points {
            // Calculate coordinate values
            let pos =
                board_rect.left_top() + vec2(2.0 * r * (x as f32) + r, 2.0 * r * (y as f32) + r);

            let mark = if go_game.last_played_pos() == Some([x, y]) {
                Mark::Dot
            } else {
                Mark::None
            };

            draw_go_piece(painter, go_game.board()[(x, y)], pos, r, mark);
        }
    }
}

fn interaction_go(
    go_game: &mut emi_go::Game,
    rect: Rect,
    painter: &egui::Painter,
    response: &Response,
) {
    let Some(pos) = response.hover_pos() else { return; };
    // Is the mouse within board bounds?
    // This assumes the board is square, which is reasonable for now, but might change.
    let rect_size = rect.width();
    if rect.contains(pos) {
        let unit = rect_size / go_game.board_size() as f32;

        let to_board =
            emath::RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, rect.size()), rect);

        // Calculate where on board
        let board_pos = to_board.inverse().transform_pos(pos);
        let point_coord_x = (board_pos.x / unit).floor() as u32;
        let point_coord_y = (board_pos.y / unit).floor() as u32;
        let hover_pos = to_board.transform_pos(Pos2::new(
            point_coord_x as f32 * unit + unit / 2.0,
            point_coord_y as f32 * unit + unit / 2.0,
        ));

        if response.clicked() {
            debug!("Trying to play at ({}, {})", point_coord_x, point_coord_y);
            let _ = go_game.play_(Move::Place {
                pos: [point_coord_x, point_coord_y],
                color: go_game.next_to_play(),
            });
        }

        // make a rectangle
        let hover_hl = Shape::rect_filled(
            Rect::from_center_size(hover_pos, vec2(unit, unit)),
            0.0,
            Color32::LIGHT_GREEN.gamma_multiply(0.5),
        );
        painter.add(hover_hl);
    }
}

#[must_use]
pub fn state_go(ctx: &egui::Context, go_game: &mut emi_go::Game) -> GoResponse {
    let resp = egui::TopBottomPanel::top("game_go_menu")
        .show(ctx, |ui| {
            ui.menu_button("Back to Main Menu", |ui| {
                if ui.button("Confirm").clicked() {
                    ui.close_menu();
                    GoResponse::MainMenu
                } else {
                    GoResponse::None
                }
            })
            .inner
            .unwrap_or(GoResponse::None)
        })
        .inner;

    egui::SidePanel::right("game_go_control")
        .show_separator_line(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(5.0);
            // Something to show who is going next
            // TODO: improve side ui
            Frame::none().outer_margin(2.0).show(ui, |ui| {
                let (response, painter) = ui.allocate_painter(vec2(50.0, 20.0), Sense::hover());
                let rect = response.rect;
                let c = rect.center();
                let radius = 8.0;
                let shift = 3.0;
                let line_width = 1.0;

                let mut color = Color32::WHITE;
                let mut opp = Color32::BLACK;

                if go_game.next_to_play() == Piece::White {
                    (opp, color) = (color, opp);
                }

                painter.add(Shape::circle_filled(c + vec2(shift, 0.0), radius, color));
                painter.add(Shape::circle_stroke(
                    c + vec2(shift, 0.0),
                    radius,
                    Stroke::new(line_width, opp.gamma_multiply(0.8)),
                ));
                painter.add(Shape::circle_filled(c - vec2(shift, 0.0), radius, opp));
                painter.add(Shape::circle_stroke(
                    c - vec2(shift, 0.0),
                    radius,
                    Stroke::new(line_width, color.gamma_multiply(0.8)),
                ));
            });

            if ui.button("Pass").clicked() {
                let _ = go_game.play_(Move::Pass);
            }

            if ui.button("Undo").clicked() {
                // This will be the api:
                // go_game.pass_turn(go_game.next_to_play());
                //
                // But for now...
                go_game.undo();
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        Frame::canvas(ui.style())
            .rounding(0.0)
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::default())
            .show(ui, |ui| {
                let width = ui.available_width();
                let height = ui.available_height().min(800.0);

                let (response, painter) = ui.allocate_painter(vec2(width, height), Sense::click());

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                    response.rect,
                );

                let board_size = width.min(height);

                let board_rect = Rect::from_center_size(
                    to_screen.transform_pos(Pos2::new(width / 2.0, height / 2.0)),
                    vec2(board_size, board_size),
                );

                // Draw go board
                draw_go_board(go_game, &painter, board_rect);
                game_go_display_pieces(go_game, &painter, board_rect);

                // Handle interaction
                interaction_go(go_game, board_rect, &painter, &response);
            });
    });

    resp
}
