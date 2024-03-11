use crate::defs;
use std::cmp::Ordering;

#[allow(unused)]
use eframe::egui::{
    Button, CollapsingHeader, Color32, Context, InnerResponse, ScrollArea, SidePanel, Stroke, Ui,
    Vec2, Widget, WidgetText,
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

        // sort whenever updated
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

        // the list
        ScrollArea::vertical().show(ui, |ui| {
            for (i, ol) in self.outlines.iter().enumerate() {
                if ol.outlines.is_empty() {
                    let btn =
                        FeedButton::new(&ol.text).highlight(self.seleted_feed == Some((i, 0)));
                    let btn = ui.add(btn);
                    if btn.clicked() {
                        self.seleted_feed = Some((i, 0));
                    }
                } else {
                    // it's a feed folder
                    CollapsingHeader::new(&ol.text).show(ui, |ui| {
                        for (j, child_ol) in ol.outlines.iter().enumerate() {
                            let btn = FeedButton::new(&child_ol.text)
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

    pub fn selected_outline(&self) -> Option<Outline> {
        if let Some((i, j)) = self.seleted_feed {
            if j == 0 {
                return Some(self.outlines[i].to_owned());
            }
            return Some(self.outlines[i].outlines[j - 1].to_owned());
        }
        None
    }
}

struct FeedButton<'a>(Button<'a>);

impl<'a> FeedButton<'a> {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        let button = Button::new(text)
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
            .rounding(3.0)
            .min_size(Vec2::new(defs::FEED_PANEL_WIDTH - 10.0, 30.0));
        Self { 0: button }
    }
    pub fn highlight(mut self, h: bool) -> Self {
        if !h {
            self.0 = self.0.fill(Color32::default());
        }
        self
    }
}

impl<'a> Widget for FeedButton<'a> {
    fn ui(self, ui: &mut Ui) -> eframe::egui::Response {
        self.0.ui(ui)
    }
}
