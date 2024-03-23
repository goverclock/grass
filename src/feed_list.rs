use crate::defs::{self, ICON_SIZE};
use std::cmp::Ordering;

use eframe::egui::{include_image, Align, Image, Layout};
#[allow(unused)]
use eframe::egui::{
    Button, CollapsingHeader, Color32, Context, InnerResponse, ScrollArea, SidePanel, Stroke, Ui,
    Vec2, Widget, WidgetText,
};
use opml::{Outline, OPML};
pub struct FeedList {
    pub outlines: Vec<Outline>,
    seleted_feed: Option<(usize, usize)>, // outlines[i][j](j >= 1) for item, outlines[i][0] for folder itself
    pub sync_btn_clicked: bool,
    pub changed_feed: bool,
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
            sync_btn_clicked: false,
            changed_feed: false,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            let sync_btn = Button::image(
                Image::new(include_image!(
                    "../assets/icon/refresh_FILL0_wght400_GRAD0_opsz24.svg"
                ))
                .max_size(ICON_SIZE),
            )
            .rounding(3.0)
            .fill(Color32::default());
            let sync_btn = ui.add(sync_btn);
            self.sync_btn_clicked = sync_btn.clicked();
        });
        ui.heading("Feeds");

        // feed list
        self.changed_feed = false;
        ScrollArea::vertical().show(ui, |ui| {
            for (i, ol) in self.outlines.iter().enumerate() {
                if ol.outlines.is_empty() {
                    let btn =
                        FeedButton::new(&ol.text).highlight(self.seleted_feed == Some((i, 0)));
                    let btn = ui.add(btn);
                    if btn.clicked() {
                        self.seleted_feed = Some((i, 0));
                        self.changed_feed = true;
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
                                self.changed_feed = true;
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
        Self(button)
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
