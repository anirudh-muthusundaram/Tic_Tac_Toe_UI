use eframe::egui::{self, Color32, Vec2, RichText};
use eframe::epi;

struct TicTacToe {
    board: [[char; 3]; 3],
    current_player: char,
    winner: Option<char>,
}

impl Default for TicTacToe {
    fn default() -> Self {
        Self {
            board: [[' '; 3]; 3],
            current_player: 'X',
            winner: None,
        }
    }
}

impl epi::App for TicTacToe {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tic Tac Toe");

            if let Some(winner) = self.winner {
                ui.label(format!("Player {} wins!", winner));
            } else {
                ui.label(format!("Current player: {}", self.current_player));
            }

            ui.separator();

            let button_size = Vec2::new(60.0, 60.0);
            let mut next_player = self.current_player;

            for row in 0..3 {
                ui.horizontal(|ui| {
                    for col in 0..3 {
                        let text = self.board[row][col].to_string();
                        let color = if text == "X" {
                            Color32::RED
                        } else if text == "O" {
                            Color32::BLUE
                        } else {
                            Color32::BLACK
                        };

                        let button = egui::Button::new(RichText::new(text).color(color))
                            .wrap(false);
                        if ui.add_sized(button_size, button).clicked() && self.board[row][col] == ' ' && self.winner.is_none() {
                            self.board[row][col] = self.current_player;
                            if self.check_winner() {
                                self.winner = Some(self.current_player);
                            } else {
                                next_player = if self.current_player == 'X' { 'O' } else { 'X' };
                            }
                        }
                    }
                });
            }

            self.current_player = next_player;

            if ui.button("Restart").clicked() {
                self.board = [[' '; 3]; 3];
                self.current_player = 'X';
                self.winner = None;
            }
        });
    }

    fn name(&self) -> &str {
        "Tic Tac Toe"
    }
}

impl TicTacToe {
    fn check_winner(&self) -> bool {
        for i in 0..3 {
            if self.board[i][0] == self.current_player
                && self.board[i][1] == self.current_player
                && self.board[i][2] == self.current_player
            {
                return true;
            }
            if self.board[0][i] == self.current_player
                && self.board[1][i] == self.current_player
                && self.board[2][i] == self.current_player
            {
                return true;
            }
        }
        if self.board[0][0] == self.current_player
            && self.board[1][1] == self.current_player
            && self.board[2][2] == self.current_player
        {
            return true;
        }
        if self.board[0][2] == self.current_player
            && self.board[1][1] == self.current_player
            && self.board[2][0] == self.current_player
        {
            return true;
        }
        false
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(TicTacToe::default()),
        options,
    );
}
