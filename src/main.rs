#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, CreationContext};
use feed_list::FeedList;
use read_list::ReadList;
mod def;
mod feed_list;
mod read_list;

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
    fl: FeedList,
    rl: ReadList,
}

impl Application {
    fn new(cc: &CreationContext) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            fl: FeedList::new(),
            rl: ReadList::new(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("RSS feed list")
            .resizable(false)
            .min_width(260.0)
            .show(ctx, |ui| self.fl.ui(ui));

        egui::SidePanel::left("RSS read list")
            .resizable(false)
            .min_width(260.0)
            .show(ctx, |ui| self.rl.ui(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RSS Content");
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

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let my_font_name = String::from("LXGW-Regular");
    fonts.font_data.insert(
        my_font_name.to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/lxgw-wenkai-v1.320/LXGWWenKai-Regular.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, my_font_name.to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, my_font_name);

    ctx.set_fonts(fonts);
}
