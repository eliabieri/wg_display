//! Converts the widget configuration to a vector of widgets for rendering
use crate::renderer::widgets::aare::Aare;
use crate::renderer::widgets::base::Widget;
use crate::renderer::widgets::bernaqua::Bernaqua;
use crate::renderer::widgets::cafete::Cafete;
use crate::renderer::widgets::public_transport::PublicTransport;
use crate::renderer::widgets::today::Today;

use common::models::WidgetConfiguration;

use super::widgets::buffet_nord::BuffetNord;

/// Converts the widget configuration to a vector of widgets using the `enabled` flag
/// # Arguments
/// * `config` - The widget configuration
/// # Returns
/// A vector of widgets
pub fn config_to_widgets(config: &WidgetConfiguration) -> Vec<Box<dyn Widget>> {
    let mut widgets: Vec<Box<dyn Widget>> = vec![];
    if config.today_config.enabled {
        widgets.push(Box::new(Today::new()));
    }
    if config.cafete_config.enabled {
        widgets.push(Box::new(Cafete::new()));
    }
    if config.aare_config.enabled {
        widgets.push(Box::new(Aare::new()));
    }
    if config.bernaqua_config.enabled {
        widgets.push(Box::new(Bernaqua::new()));
    }
    if config.public_transport_config.base_config.enabled {
        widgets.push(Box::new(PublicTransport::new()));
    }
    if config.buffet_nord_config.enabled {
        widgets.push(Box::new(BuffetNord::new()));
    }
    widgets
}
