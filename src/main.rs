// src/main.rs
use eframe::{egui, run_simple_native, NativeOptions};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Cherry,
    Lemon,
    Orange,
    Plum,
    Bell,
    Bar,
}

impl Symbol {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..6) {
            0 => Symbol::Cherry,
            1 => Symbol::Lemon,
            2 => Symbol::Orange,
            3 => Symbol::Plum,
            4 => Symbol::Bell,
            _ => Symbol::Bar,
        }
    }

    fn emoji(self) -> &'static str {
        match self {
            Symbol::Cherry => "🍒",
            Symbol::Lemon  => "🍋",
            Symbol::Orange => "🍊",
            Symbol::Plum   => "🍇",
            Symbol::Bell   => "🔔",
            Symbol::Bar    => "🍫",
        }
    }
}

fn main() -> eframe::Result<()> {
    // State: the three reels + whether we just won
    let mut reels = [
        Symbol::random(),
        Symbol::random(),
        Symbol::random(),
    ];
    let mut won = false;

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 220.0)),
        ..Default::default()
    };

    run_simple_native(
        "Rusty Slot Machine",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("🎰 Slot Machine 🎰");

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 20.0;
                    for &sym in &reels {
                        ui.label(egui::RichText::new(sym.emoji()).size(64.0));
                    }
                });

                ui.add_space(8.0);

                if ui.button("Spin").clicked() {
                    reels = [
                        Symbol::random(),
                        Symbol::random(),
                        Symbol::random(),
                    ];
                    // Check for win
                    won = reels[0] == reels[1] && reels[1] == reels[2];
                }

                // Show a message if the player won
                if won {
                    ui.add_space(8.0);
                    ui.colored_label(egui::Color32::from_rgb(0, 200, 0), "🎉 You win! 🎉");
                }
            });
        },
    )
}
