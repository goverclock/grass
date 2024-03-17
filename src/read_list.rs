use eframe::egui::{Button, Color32, Context, ScrollArea, Stroke, Ui, Vec2};
use opml::Outline;
use rss::Channel;
use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
    thread,
};

#[derive(PartialEq, Eq)]
struct HashOutline(Outline);
impl Hash for HashOutline {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.xml_url.hash(state)
    }
}

pub struct ReadList {
    all_items: Arc<Mutex<HashMap<HashOutline, Vec<rss::Item>>>>, // fetched item for each outline, sorted by time(newest first)
    selected_item: Option<usize>,
    showing_outlines: Option<Outline>,

    ctx: Context,
}

impl ReadList {
    pub fn new(ctx: Context) -> Self {
        Self {
            selected_item: None,
            all_items: Arc::new(Mutex::new(HashMap::new())),
            showing_outlines: None,
            ctx,
        }
    }

    // set the outlines for showing
    pub fn set_outline(&mut self, ol: Option<Outline>) {
        self.showing_outlines = ol
    }

    pub fn fetch_item(&mut self, outlines: &Vec<Outline>) {
        println!("fetch BEGIN");

        fn fetch(
            ctx: Context,
            ol: Outline,
            results: Arc<Mutex<HashMap<HashOutline, Vec<rss::Item>>>>,
        ) {
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

            results
                .lock()
                .unwrap()
                .insert(HashOutline(ol.to_owned()), fetched_items);

            if Arc::strong_count(&results) == 2 {
                println!("fetch DONE, repainting");
                ctx.request_repaint();
            }
        }

        for ol in outlines {
            if ol.outlines.is_empty() {
                let result = Arc::clone(&self.all_items);
                let ol = ol.to_owned();
                let ctx = self.ctx.clone();
                thread::spawn(move || {
                    fetch(ctx, ol, result);
                });
                continue;
            }
            for child_ol in ol.outlines.iter() {
                let result = Arc::clone(&self.all_items);
                let ol = child_ol.to_owned();
                let ctx = self.ctx.clone();
                thread::spawn(move || {
                    fetch(ctx, ol, result);
                });
            }
        }
    }

    pub fn is_fetching(&self) -> bool {
        Arc::strong_count(&self.all_items) != 1
    }

    pub fn selected_item(&self) -> Option<rss::Item> {
        let ind = self.selected_item?;
        let showing_items = self.get_showing_items();
        Some(showing_items[ind].clone())
    }

    // calculate showing items base on selected outlines
    fn get_showing_items(&self) -> Vec<rss::Item> {
        let mut showing_items = vec![]; // TODO: can this be Vec::<&Item>?
        let mut gather_items = |ol: &Outline| {
            self.all_items
                .lock()
                .unwrap()
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
        showing_items
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("All"); // All, Unread, Starred
        });
        let size = ui.min_size();

        let showing_items = self.get_showing_items();
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
