#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::plot::{Line, Plot, PlotPoints};
use std::time::Instant;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "I am watching this for the plot",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    start: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            allowed_to_close: false,
            show_confirmation_dialog: false,
            start: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("Try to close the window");

            let sin: PlotPoints = (0..10000)
                .map(|i| {
                    let x = i as f64 * 0.01 + self.start.elapsed().as_secs_f64();
                    [x, x.sin()]
                })
                .collect();
            let line = Line::new(sin);
            Plot::new("my_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        });

        if self.show_confirmation_dialog {
            // Show confirmation dialog:
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.show_confirmation_dialog = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.allowed_to_close = true;
                            frame.close();
                        }
                    });
                });
        }
        ctx.request_repaint();
    }
}
