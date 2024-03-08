use crate::def;
use eframe::egui::{Button, Color32, ScrollArea, Sense, Stroke, Ui, Vec2};
use opml::OPML;
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
            let title = self.opml.head.as_ref().unwrap().title.as_ref().unwrap();
            ui.heading(title);
        });
        ScrollArea::vertical().show(ui, |ui| {
            let outlines = &self.opml.body.outlines;
            for ol in outlines {
                ui.label(&ol.text);
            }
        });
    }
}
