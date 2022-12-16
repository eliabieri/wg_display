use std::thread;
use std::time::Duration;

use common::models::SystemConfiguration;
use cursive::theme::BaseColor;
use cursive::theme::BorderStyle;
use cursive::theme::Color;
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
        self.initialize_layout(&config, &mut siv);

        loop {
            if let Some(new_config) = Persistence::get_config_change() {
                config = new_config;
                self.initialize_layout(&config, &mut siv)
            }

            self.update_widgets(&mut siv, &config.widget_config).await;
            siv.step();
            siv.refresh();
            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn initialize_layout(
        &mut self,
        config: &SystemConfiguration,
        siv: &mut CursiveRunner<CursiveRunnable>,
    ) {
        let widgets = config_to_widgets(&config.widget_config);
        self.widgets = widgets;
        *siv = cursive::default().into_runner();
        siv.update_theme(|theme| theme.shadow = false);
        siv.update_theme(|theme| {
            theme.palette[Background] =
                Color::parse(config.background_color.as_str()).unwrap_or(Dark(BaseColor::Magenta))
        });
        siv.update_theme(|theme| theme.borders = BorderStyle::None);
        siv.add_layer(PaddedView::lrtb(1, 1, 0, 0, self.build_layout()));
    }

    fn build_layout(&self) -> Panel<PaddedView<LinearLayout>> {
        let mut linear_layout = LinearLayout::vertical();
        self.widgets.iter().for_each(|widget| {
            let name_widget = LinearLayout::horizontal().child(TextView::new(format!(
                "{:width$}",
                widget.get_meta_data().name(),
                width = self.name_column_width()
            )));

            let content_widget =
                TextView::new(widget.get_content()).with_name(widget.get_meta_data().name());

            let padded_view = name_widget.child(content_widget);
            linear_layout.add_child(padded_view);
        });

        Panel::new(PaddedView::lrtb(0, 0, 1, 0, linear_layout)).title("wgdisplay.local")
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
            siv.call_on_name(widget.get_meta_data().name(), |view: &mut TextView| {
                view.set_content(widget.get_content());
            });
        });
    }

    fn name_column_width(&self) -> usize {
        self.widgets
            .iter()
            .map(|widget| widget.get_meta_data().name().len())
            .max()
            .unwrap()
            + 2
    }
}
