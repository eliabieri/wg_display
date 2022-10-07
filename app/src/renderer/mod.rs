use std::{thread, time::Duration};

use cursive::{
    view::Nameable,
    views::{Dialog, LinearLayout, TextView},
};

use crate::shared::persistence::Persistence;

pub struct Renderer {}

impl Renderer {
    pub fn render() {
        let mut siv = cursive::default().into_runner();
        siv.add_layer(
            Dialog::around(
                LinearLayout::horizontal()
                    .child(TextView::new("Example value: ").with_name("title"))
                    .child(
                        TextView::new(Persistence::get_config().unwrap().example_value)
                            .with_name("text"),
                    ),
            )
            .title("WG Display"),
        );

        loop {
            siv.call_on_name("text", |view: &mut TextView| {
                view.set_content(Persistence::get_config().unwrap().example_value);
            });
            siv.step();
            siv.refresh();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
