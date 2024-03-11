use std::hash::Hash;

use eframe::egui::{
    ahash::{HashMap, HashMapExt},
    Button, Color32, ScrollArea, Stroke, Ui, Vec2,
};
use opml::Outline;

#[derive(PartialEq, Eq)]
struct HashOutline(Outline);
impl Hash for HashOutline {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.xml_url.hash(state)
    }
}

pub struct ReadList {
    feed_items: HashMap<HashOutline, Vec<rss::Item>>, // fetched item for each outline, sorted by time(newest first)
    selected_item: Option<usize>,
    showing_outlines: Option<Outline>,
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
        let hm = HashMap::new();

        Self {
            selected_item: None,
            feed_items: hm,
            showing_outlines: None,
        }
    }

    pub fn set_outline(&mut self, ol: Option<Outline>) {
        self.showing_outlines = ol
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("All"); // All, Unread, Starred
        });
        let size = ui.min_size();

        let mut showing_items = vec![]; // TODO: can this be Vec::<&Item>?
        if let Some(ols) = &self.showing_outlines {
            if ols.outlines.len() == 0 {
                // single outline
                self.feed_items
                    .entry(HashOutline(ols.to_owned()))
                    .or_insert(vec![])
                    .iter()
                    .for_each(|i| {
                        showing_items.push(i.to_owned());
                    });
            } else {
                // outline folder
                for ol in ols.outlines.iter() {
                    self.feed_items
                        .entry(HashOutline(ol.to_owned()))
                        .or_insert(vec![])
                        .iter()
                        .for_each(|i| {
                            showing_items.push(i.to_owned());
                        });
                }
            }
        }

        ScrollArea::vertical().show(ui, |ui| {
            for (i, fi) in showing_items.iter().enumerate() {
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
