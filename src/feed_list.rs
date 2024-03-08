use std::cmp::Ordering;

use crate::def;
use eframe::egui::{Button, CollapsingHeader, Color32, Order, ScrollArea, Sense, Stroke, Ui, Vec2};
use opml::{Outline, OPML};
pub struct FeedList {
    opml: OPML,
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

        Self { opml }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Feeds");
        });

        ScrollArea::vertical().show(ui, |ui| {
            self.opml.body.outlines.sort_by(|a, b| {
                let a_is_folder = a.outlines.len() != 0;
                let b_is_folder = b.outlines.len() != 0;
                if a_is_folder == b_is_folder {
                    Ordering::Equal
                } else if a_is_folder {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            for ol in &self.opml.body.outlines {
                if ol.outlines.len() == 0 {
                    ui.label(&ol.text);
                } else {
                    CollapsingHeader::new(&ol.text)
                        .default_open(false)
                        .show(ui, |ui| {
                            for child_ol in &ol.outlines {
                                ui.label(&child_ol.text);
                            }
                        });
                }
            }
        });
    }
}
