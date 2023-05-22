//! Widgets to display rendering implementation using [Cursive](https://crates.io/crates/cursive)
use std::thread;
use std::time::Duration;

use common::models::SystemConfiguration;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::Color::Dark;
use cursive::theme::PaletteColor::Background;
use cursive::theme::Style;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::PaddedView;
use cursive::views::{LinearLayout, TextView};
use cursive::{CursiveRunnable, CursiveRunner};

use crate::shared::persistence::Persistence;

use crate::shared::widget_manager::WidgetManager;
use crate::widgets::running::runtime::Runtime;
use crate::widgets::running::runtime::Widget;

struct WasmWidget {
    name: String,
    widget: Widget,
}

pub struct Renderer {
    widgets: Vec<WasmWidget>,
    runtime: Runtime,
}

// Renders the widget on the display using the [Cursive](https://crates.io/crates/cursive) crate
impl Renderer {
    pub fn new() -> Self {
        let mut runtime = Runtime::new();
        Self {
            widgets: Renderer::initialize_widgets(&mut runtime),
            runtime,
        }
    }

    fn initialize_widgets(runtime: &mut Runtime) -> Vec<WasmWidget> {
        let mut widgets = vec![];
        for widget in WidgetManager::get_widgets() {
            let widget = runtime.instantiate_widget(&widget);
            if let Ok(widget) = widget {
                let name = runtime
                    .get_widget_name(&widget)
                    .expect("Could not get widget name");

                widgets.push(WasmWidget { name, widget });
            }
        }
        widgets
    }

    /// Runs the renderer (blocking)
    pub fn run(&mut self) {
        let mut siv = cursive::default().into_runner();
        let mut config = Persistence::get_system_config().expect("Could not load config");
        self.initialize_layout(&config, &mut siv);

        loop {
            if let Some(new_config) = Persistence::get_system_config_change() {
                config = new_config;
                self.widgets = Renderer::initialize_widgets(&mut self.runtime);
                self.initialize_layout(&config, &mut siv)
            }

            self.update_widgets(&mut siv);
            siv.step();
            siv.refresh();

            thread::sleep(Duration::from_millis(1000));
        }
    }

    /// Initializes the layout using given system configuration
    /// # Args
    /// * `config` - The system configuration
    /// * `siv` - The cursive instance
    fn initialize_layout(
        &mut self,
        config: &SystemConfiguration,
        siv: &mut CursiveRunner<CursiveRunnable>,
    ) {
        *siv = cursive::default().into_runner();
        siv.update_theme(|theme| theme.shadow = false);
        siv.update_theme(|theme| {
            theme.palette[Background] =
                Color::parse(config.background_color.as_str()).unwrap_or(Dark(BaseColor::Magenta))
        });
        siv.add_layer(self.build_layout());
    }

    /// Builds the layout
    /// # Returns
    /// The layout as Panel
    fn build_layout(&self) -> LinearLayout {
        let mut linear_layout = LinearLayout::vertical();

        let title = TextView::new(Renderer::get_title())
            .style(Style::title_primary())
            .center();
        linear_layout.add_child(title.full_width());

        self.widgets.iter().for_each(|widget| {
            let name_widget = TextView::new(widget.name.clone()).style(Style::title_secondary());

            let content_widget = TextView::new("-".to_string()).with_name(widget.name.clone());

            linear_layout.add_child(PaddedView::lrtb(1, 0, 0, 0, name_widget));
            linear_layout.add_child(PaddedView::lrtb(1, 0, 0, 0, content_widget));
        });
        linear_layout
    }

    /// Calls the update function on all enabled widgets
    /// # Args
    /// * `siv` - The cursive instance
    /// * `config` - The widget configuration
    fn update_widgets(&mut self, siv: &mut CursiveRunner<CursiveRunnable>) {
        self.widgets.iter_mut().for_each(|widget| {
            let widget_config = Persistence::get_widget_config(widget.name.as_str());
            let widget_config = widget_config.unwrap_or("{}".to_string());

            let res = self.runtime.run_widget(&widget.widget, &widget_config);
            let res = match res {
                Ok(res) => res.map(|res| res.data),
                Err(err) => Some(err.to_string()),
            };

            if let Some(data) = res {
                siv.call_on_name(widget.name.as_str(), |view: &mut TextView| {
                    view.set_content(data);
                });
            }
        });
    }

    /// Computes the title of the application panel
    /// # Returns
    /// The title of the application panel
    fn get_title() -> String {
        match hostname::get() {
            Ok(hostname) => match hostname.into_string() {
                Ok(hostname) => format!("{}.local", hostname),
                Err(_) => "hostname format error".into(),
            },
            Err(_) => "unkown hostname".into(),
        }
    }
}
