#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, CreationContext};
use feed_list::FeedList;
mod def;
mod feed_list;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    //     ..Default::default()
    // };
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "grass",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx); // this gives us image support
            Box::new(Application::new(cc))
        }),
    )
}

struct Application {
    // name: String,
    // age: u32,
    fl: FeedList,
}

impl Application {
    fn new(_cc: &CreationContext) -> Self {
        Self {
            fl: FeedList::new(),
        }
    }
}

// impl Default for Application {
//     fn default() -> Self {
//         Self {
//             // name: "Arthur".to_owned(),
//             // age: 42,
//         }
//     }
// }

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("rss source groups")
            .resizable(false)
            .min_width(260.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Feeds(rss source groups)");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(def::LOREM_IPSUM_LONG);
                    ui.label(def::LOREM_IPSUM_LONG);
                    ui.label(def::LOREM_IPSUM_LONG);
                    ui.label(def::LOREM_IPSUM_LONG);
                });
            });

        egui::SidePanel::left("rss feed list")
            .resizable(false)
            .min_width(260.0)
            .show(ctx, |ui| self.fl.ui(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("there should be rss contents");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Increment").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!("../ferris.png"));
        });
    }
}
