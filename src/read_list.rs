use eframe::egui::{Button, Color32, ScrollArea, Sense, Stroke, Ui, Vec2};
use rss::Channel;
pub struct ReadList {
    selected_feed: Option<usize>,
    // feeds: Vec<>
    feed_items: Vec<rss::Item>,
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
            selected_feed: None,
            feed_items: vec![],
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Unread");
        });
        let size = ui.min_size();
        ScrollArea::vertical().show(ui, |ui| {
            let fi = &self.feed_items;
            for i in 0..fi.len() {
                let mut btn = Button::new(fi[i].title().unwrap_or("titleless feed"))
                    .sense(Sense::click())
                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                    .rounding(3.0)
                    .min_size(Vec2::new(size.x - 10.0, 110.0));
                if let Some(s) = self.selected_feed {
                    if s != i {
                        btn = btn.fill(Color32::default())
                    }
                } else {
                    btn = btn.fill(Color32::default())
                }

                let resp = ui.add(btn);
                if resp.clicked() {
                    self.selected_feed = Some(i);
                }
            }
        });
    }
}
