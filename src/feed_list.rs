use eframe::egui::{Button, Color32, ScrollArea, Sense, Stroke, Ui, Vec2};
pub struct FeedList {
    selected_feed: Option<usize>,
}

impl FeedList {
    pub fn new() -> Self {
        Self { selected_feed: None }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("there should be rss feeds");
        let size = ui.min_size();
        ScrollArea::vertical().show(ui, |ui| {
            for i in 0..10 {
                let mut btn = Button::new("Fuck")
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
