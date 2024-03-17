use eframe::egui::Ui;

pub struct Content {
    item: Option<rss::Item>,
}

impl Content {
    pub fn new() -> Self {
        Self { item: None }
    }

    pub fn show_item(&mut self, item: Option<rss::Item>) {
        self.item = item
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("RSS Content");
        // ui.image(egui::include_image!("../assets/ferris.png"));
        if self.item.is_none() {
            return;
        }
        let text = self.item.as_ref().unwrap().description.as_ref().unwrap();
        ui.label(text);
    }
}
