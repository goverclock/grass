#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, CreationContext};
use feed_list::FeedList;
use read_list::ReadList;
mod defs;
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
    feed_list: FeedList,
    read_list: ReadList,
}

impl Application {
    fn new(cc: &CreationContext) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            feed_list: FeedList::new(),
            read_list: ReadList::new(),
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: FeedPanel
        egui::SidePanel::left("RSS feed panel")
            .resizable(false)
            .min_width(defs::FEED_PANEL_WIDTH)
            .max_width(defs::FEED_PANEL_WIDTH)
            .show(ctx, |ui| self.feed_list.ui(ui));

        egui::SidePanel::left("RSS read panel")
            .resizable(false)
            .min_width(defs::READ_PANEL_WIDTH)
            .max_width(defs::READ_PANEL_WIDTH)
            .show(ctx, |ui| self.read_list.ui(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RSS Content");
            ui.image(egui::include_image!("../assets/ferris.png"));
        });

        let fl = &mut self.feed_list;
        let rl = &mut self.read_list;

        rl.set_outline(fl.selected_outline());

        if fl.sync_btn_clicked {
            rl.fetch_item(&fl.outlines);
        }
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
