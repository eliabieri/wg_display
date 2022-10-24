use std::thread;
use std::time::Duration;

use cursive::view::Nameable;
use cursive::views::{LinearLayout, PaddedView, TextView};
use futures::future::join_all;
use rocket::tokio::join;

use crate::shared::persistence::Persistence;

mod widgets;
use crate::renderer::widgets::base::Widget;
use crate::renderer::widgets::cafete::Cafete;
use crate::renderer::widgets::time::Time;

pub struct Renderer {
    widgets: Vec<Box<dyn Widget>>,
}

impl Renderer {
    pub fn new() -> Self {
        let _config = Persistence::get_config().expect("Could not load config");
        // TODO: use config to determine which widgets shall be instantiated
        Self {
            widgets: vec![Box::new(Time::new()), Box::new(Cafete::new())],
        }
    }

    pub async fn run(&mut self) {
        let mut siv = cursive::default().into_runner();
        let mut names = LinearLayout::vertical();
        let mut content = LinearLayout::vertical();
        self.widgets.iter().for_each(|widget| {
            names.add_child(TextView::new(format!("{}  ", widget.get_name())));
            content.add_child(TextView::new(widget.get_content()).with_name(widget.get_name()));
        });
        siv.add_layer(PaddedView::lrtb(
            2,
            2,
            0,
            0,
            LinearLayout::horizontal().child(names).child(content),
        ));

        loop {
            let config = Persistence::get_config().expect("Could not load config");
            join!(join_all(
                self.widgets.iter_mut().map(|widget| widget.update(&config))
            ));

            self.widgets.iter_mut().for_each(|widget| {
                siv.call_on_name(widget.get_name(), |view: &mut TextView| {
                    view.set_content(widget.get_content());
                });
            });
            siv.step();
            siv.refresh();
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
