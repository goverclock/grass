use eframe::egui::{Button, Color32, ScrollArea, Sense, Stroke, Ui, Vec2};
use rss::Channel;
pub struct FeedList {
    selected_feed: Option<usize>,
    // feeds: Vec<>
    feed_items: Vec<rss::Item>,
}

impl FeedList {
    pub fn new() -> Self {
        // initial feeds
        // let f = std::fs::File::open("twit.xml").unwrap();
        // let chan = Channel::read_from(BufReader::new(f)).unwrap();
        // println!("{:?}", chan.items().len());

        let content = reqwest::blocking::get("https://feeds.twit.tv/twit.xml")
            .unwrap()
            .bytes()
            .unwrap();
        let chan = Channel::read_from(&content[..]).unwrap();
        let feed_items = chan.into_items();

        Self {
            selected_feed: None,
            feed_items,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("there should be rss feeds");
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
