//! Widgets to display rendering implementation using [Cursive](https://crates.io/crates/cursive)
use std::fs;
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

use crate::shared::persistence::Persistence;

use crate::widgets::running::runtime::Plugin;
use crate::widgets::running::runtime::Runtime;
use crate::widgets::utils::loader::Loader;

struct WasmWidget {
    name: String,
    plugin: Plugin,
}

pub struct Renderer {
    widgets: Vec<WasmWidget>,
    runtime: Runtime,
}

// Renders the widget on the display using the [Cursive](https://crates.io/crates/cursive) crate
impl Renderer {
    pub fn new() -> Self {
        // let config = Persistence::get_config().expect("Could not load config");
        let mut runtime = Runtime::new();
        Self {
            widgets: Renderer::initialize_widgets(&mut runtime),
            runtime,
        }
    }

    fn get_widget_paths() -> Vec<String> {
        let widgets_folder = fs::read_dir("widgets").expect("Could not read widgets directory");
        widgets_folder
            .filter_map(|entry| {
                let entry = entry.expect("Could not read entry");
                let path = entry.path();
                if path.is_dir() {
                    let path = path.to_str().expect("Could not convert path to string");
                    Some(path.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    fn initialize_widgets(runtime: &mut Runtime) -> Vec<WasmWidget> {
        let widget_paths = Renderer::get_widget_paths();
        widget_paths
            .iter()
            .map(|path| {
                let component_binary =
                    Loader::load_core_module_as_component(format!("{}/plugin.wasm", path).as_str())
                        .expect("Could not load WASM module");

                let plugin = runtime
                    .instantiate_plugin(component_binary)
                    .expect("Could not instantiate plugin");
                let name = runtime
                    .get_plugin_name(&plugin)
                    .expect("Could not get plugin name");

                WasmWidget { name, plugin }
            })
            .collect()
    }

    /// Runs the renderer (blocking)
    pub fn run(&mut self) {
        let mut siv = cursive::default().into_runner();
        let mut config = Persistence::get_config().expect("Could not load config");
        self.initialize_layout(&config, &mut siv);

        loop {
            if let Some(new_config) = Persistence::get_config_change() {
                config = new_config;

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
        siv.update_theme(|theme| theme.borders = BorderStyle::None);
        siv.add_layer(PaddedView::lrtb(1, 1, 0, 0, self.build_layout()));
    }

    /// Builds the layout
    /// # Returns
    /// The layout as Panel
    fn build_layout(&self) -> Panel<PaddedView<LinearLayout>> {
        let mut linear_layout = LinearLayout::vertical();
        self.widgets.iter().for_each(|widget| {
            let name_widget = LinearLayout::horizontal().child(TextView::new(format!(
                "{:width$}",
                widget.name,
                width = self.name_column_width()
            )));

            let content_widget = TextView::new("-".to_string()).with_name(widget.name.clone());

            let padded_view = name_widget.child(content_widget);
            linear_layout.add_child(padded_view);
        });

        let title = Renderer::get_title();
        Panel::new(PaddedView::lrtb(0, 0, 1, 0, linear_layout)).title(title)
    }

    /// Calls the update function on all enabled widgets
    /// # Args
    /// * `siv` - The cursive instance
    /// * `config` - The widget configuration
    fn update_widgets(&mut self, siv: &mut CursiveRunner<CursiveRunnable>) {
        self.widgets.iter_mut().for_each(|widget| {
            let widget_config = Persistence::get_widget_config(widget.name.as_str());
            let widget_config = widget_config.unwrap_or("{}".to_string());

            let res = self.runtime.run_plugin(&widget.plugin, &widget_config);
            let res = match res {
                Ok(res) => res.data,
                Err(e) => "Error while running plugin".into(),
            };
            siv.call_on_name(widget.name.as_str(), |view: &mut TextView| {
                view.set_content(res);
            });
        });
    }

    /// Calculates the width of the name column
    /// # Returns
    /// The safe width of the name column
    fn name_column_width(&self) -> usize {
        self.widgets
            .iter()
            .map(|widget| widget.name.len())
            .max()
            .unwrap()
            + 2
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
