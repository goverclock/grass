use std::cmp::Ordering;
use crate::defs;

use eframe::egui::{
    Button, CollapsingHeader, Color32, ScrollArea, Stroke, Ui, Vec2, Widget, WidgetText,
};
use opml::{Outline, OPML};
pub struct FeedList {
    outlines: Vec<Outline>,
    seleted_feed: Option<(usize, usize)>, // outlines[i][j](j >= 1) for item, outlines[i][0] for folder itself
}

impl FeedList {
    pub fn new() -> Self {
        let mut f = std::fs::File::open("Feeds.opml").unwrap();
        let opml = OPML::from_reader(&mut f).unwrap();
        // println!("{:#?}", document.head.unwrap());
        // println!("{:#?}", document.body.outlines[0]);
        // for ol in document.body.outlines {
        //     if ol.title == Some("Info".to_string()) {
        //         println!("{:#?}", ol.outlines);  // multi level outlines
        //     }
        // }
        // TODO: check opml.head.unwrap here
        let mut outlines = opml.body.outlines;

        outlines.sort_by(|a, b| {
            let a_is_folder = !a.outlines.is_empty();
            let b_is_folder = !b.outlines.is_empty();
            if a_is_folder == b_is_folder {
                Ordering::Equal
            } else if a_is_folder {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        Self {
            outlines,
            seleted_feed: None,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Feeds");
        });

        ScrollArea::vertical().show(ui, |ui| {
            for (i, ol) in self.outlines.iter().enumerate() {
                if ol.outlines.is_empty() {
                    let btn =
                        ItemButton::new(&ol.text).highlight(self.seleted_feed == Some((i, 1)));
                    let btn = ui.add(btn);
                    if btn.clicked() {
                        self.seleted_feed = Some((i, 1));
                    }
                } else {
                    // it's a feed folder
                    CollapsingHeader::new(&ol.text).show(ui, |ui| {
                        for (j, child_ol) in ol.outlines.iter().enumerate() {
                            let btn = ItemButton::new(&child_ol.text)
                                .highlight(self.seleted_feed == Some((i, j + 1)));
                            let btn = ui.add(btn);
                            if btn.clicked() {
                                self.seleted_feed = Some((i, j + 1));
                            }
                        }
                    });
                }
            }
        });
    }
}

struct ItemButton<'a> {
    button: Button<'a>,
}

impl<'a> ItemButton<'a> {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        let button = Button::new(text)
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
            .rounding(3.0)
            .min_size(Vec2::new(defs::FEED_PANEL_WIDTH - 10.0, 30.0));
        Self { button }
    }
    pub fn highlight(mut self, h: bool) -> Self {
        if !h {
            self.button = self.button.fill(Color32::default());
        }
        self
    }
}

impl<'a> Widget for ItemButton<'a> {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        self.button.ui(ui)
    }
}
