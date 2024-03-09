use eframe::egui::{Button, Color32, ScrollArea, Stroke, Ui, Vec2};

pub struct ReadList {
    feed_items: Vec<rss::Item>,
    selected_item: Option<usize>,
}

impl ReadList {
    pub fn new() -> Self {
        // initial feeds
        // TODO: do not use blocking, use async, remember to update Cargo.toml to remove blocking feature
        // let content = reqwest::blocking::get("https://feeds.twit.tv/twit.xml")
        //     .unwrap()
        //     .bytes()
        //     .unwrap();
        // let chan = Channel::read_from(&content[..]).unwrap();
        // let feed_items = chan.into_items();

        Self {
            selected_item: None,
            feed_items: vec![],
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("All"); // All, Unread, Starred
        });
        let size = ui.min_size();
        ScrollArea::vertical().show(ui, |ui| {
            let _fi = &self.feed_items;
            for (i, fi) in self.feed_items.iter().enumerate() {
                let mut btn = Button::new(fi.title().unwrap_or("titleless feed"))
                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                    .rounding(3.0)
                    .min_size(Vec2::new(size.x - 10.0, 110.0));
                if let Some(s) = self.selected_item {
                    if s != i {
                        btn = btn.fill(Color32::default())
                    }
                } else {
                    btn = btn.fill(Color32::default())
                }

                let btn = ui.add(btn);
                if btn.clicked() {
                    self.selected_item = Some(i);
                }
            }
        });
    }
}
