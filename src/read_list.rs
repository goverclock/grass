use eframe::egui::{Button, Color32, ScrollArea, Stroke, Ui, Vec2};
use opml::Outline;
use rss::Channel;
use std::{collections::HashMap, hash::Hash};

#[derive(PartialEq, Eq)]
struct HashOutline(Outline);
impl Hash for HashOutline {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.xml_url.hash(state)
    }
}

pub struct ReadList {
    all_items: HashMap<HashOutline, Vec<rss::Item>>, // fetched item for each outline, sorted by time(newest first)
    selected_item: Option<usize>,
    showing_outlines: Option<Outline>,
}

impl ReadList {
    pub fn new() -> Self {
        Self {
            selected_item: None,
            all_items: HashMap::new(),
            showing_outlines: None,
        }
    }

    pub fn set_outline(&mut self, ol: Option<Outline>) {
        self.showing_outlines = ol
    }

    pub fn fetch_item(&mut self, outlines: &Vec<Outline>) {
        // TODO: do not use blocking, use async, remember to update Cargo.toml to remove blocking feature
        // let content = reqwest::blocking::get("https://feeds.twit.tv/twit.xml")
        //     .unwrap()
        //     .bytes()
        //     .unwrap();
        // let chan = Channel::read_from(&content[..]).unwrap();
        // let feed_items = chan.into_items();

        let mut fetch = |ol: &Outline| {
            if ol.xml_url.is_none() {
                println!("fail to fetch {}", ol.text);
                return;
            }
            let url = ol.xml_url.as_ref().unwrap();
            println!("fetching: {}", url);
            let content = reqwest::blocking::get(url).unwrap().bytes().unwrap();
            let chan = Channel::read_from(&content[..]);
            if chan.is_err() {
                println!("fial to read_from {}", url);
                return;
            }
            let chan = chan.unwrap();
            let fetched_items = chan.into_items();
            println!("fetched {}", fetched_items.len());

            self.all_items
                .insert(HashOutline(ol.to_owned()), fetched_items);
        };

        for ol in outlines {
            if ol.outlines.is_empty() {
                fetch(ol);
                continue;
            }
            for child_ol in ol.outlines.iter() {
                fetch(child_ol);
            }
        }

        println!("fetch done");
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("All"); // All, Unread, Starred
        });
        let size = ui.min_size();

        let mut showing_items = vec![]; // TODO: can this be Vec::<&Item>?
        let mut gather_items = |ol: &Outline| {
            self.all_items
                .entry(HashOutline(ol.to_owned()))
                .or_default()
                .iter()
                .for_each(|i| {
                    showing_items.push(i.to_owned());
                });
        };

        if let Some(ols) = &self.showing_outlines {
            if ols.outlines.is_empty() {
                // single outline
                gather_items(ols);
            } else {
                // outline folder
                for ol in ols.outlines.iter() {
                    gather_items(ol);
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
