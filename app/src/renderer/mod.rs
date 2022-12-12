use std::thread;
use std::time::Duration;

use cursive::theme::BaseColor::Green;
use cursive::theme::Color::Dark;
use cursive::theme::PaletteColor::Background;
use cursive::view::Nameable;
use cursive::views::{LinearLayout, PaddedView, Panel, TextView};
use cursive::{CursiveRunnable, CursiveRunner};
use futures::future::join_all;
use rocket::tokio::join;

use common::models::WidgetConfiguration;

use crate::shared::persistence::Persistence;

mod widgets;
use crate::renderer::widgets::base::Widget;

mod config_to_widgets;
use config_to_widgets::config_to_widgets;

pub struct Renderer {
    widgets: Vec<Box<dyn Widget>>,
}

impl Renderer {
    pub fn new() -> Self {
        let config = Persistence::get_config().expect("Could not load config");
        Self {
            widgets: config_to_widgets(&config.widget_config),
        }
    }

    /// Runs the renderer (blocking)
    pub async fn run(&mut self) {
        let mut siv = cursive::default().into_runner();
        let mut config = Persistence::get_config().expect("Could not load config");
        self.initialize_layout(&config.widget_config, &mut siv);

        loop {
            if let Some(new_config) = Persistence::get_config_change() {
                config = new_config;
                self.initialize_layout(&config.widget_config, &mut siv)
            }

            self.update_widgets(&mut siv, &config.widget_config).await;
            siv.step();
            siv.refresh();
            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn initialize_layout(
        &mut self,
        config: &WidgetConfiguration,
        siv: &mut CursiveRunner<CursiveRunnable>,
    ) {
        let widgets = config_to_widgets(config);
        self.widgets = widgets;
        *siv = cursive::default().into_runner();
        siv.update_theme(|theme| theme.shadow = false);
        siv.update_theme(|theme| theme.palette[Background] = Dark(Green));
        siv.add_layer(PaddedView::lrtb(1, 1, 0, 0, self.build_layout()));
    }

    fn build_layout(&self) -> Panel<LinearLayout> {
        let mut linear_layout = LinearLayout::vertical();
        self.widgets.iter().for_each(|widget| {
            let name_widget = LinearLayout::horizontal().child(TextView::new(format!(
                "{:width$}",
                widget.get_name().as_str(),
                width = self.name_column_width()
            )));

            let content_widget =
                TextView::new(widget.get_content()).with_name(widget.get_name().as_str());

            let padded_view = name_widget.child(content_widget);
            linear_layout.add_child(padded_view);
        });

        Panel::new(linear_layout).title("WG Display")
    }

    async fn update_widgets(
        &mut self,
        siv: &mut CursiveRunner<CursiveRunnable>,
        config: &WidgetConfiguration,
    ) {
        join!(join_all(
            self.widgets.iter_mut().map(|widget| widget.update(config))
        ));

        self.widgets.iter_mut().for_each(|widget| {
            siv.call_on_name(widget.get_name().as_str(), |view: &mut TextView| {
                view.set_content(widget.get_content());
            });
        });
    }

    fn name_column_width(&self) -> usize {
        self.widgets
            .iter()
            .map(|widget| widget.get_name().as_str().len())
            .max()
            .unwrap()
            + 2
    }
}
