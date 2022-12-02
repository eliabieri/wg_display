use crate::renderer::widgets::aare::Aare;
use crate::renderer::widgets::base::Widget;
use crate::renderer::widgets::bernaqua::Bernaqua;
use crate::renderer::widgets::cafete::Cafete;
use crate::renderer::widgets::public_transport::PublicTransport;
use crate::renderer::widgets::time::Time;

use common::models::WidgetConfiguration;

pub fn config_to_widgets(config: &WidgetConfiguration) -> Vec<Box<dyn Widget>> {
    let mut widgets: Vec<Box<dyn Widget>> = vec![];
    if config.time_config.enabled {
        widgets.push(Box::new(Time::new()));
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
    if config.public_transport_config.enabled {
        widgets.push(Box::new(PublicTransport::new()));
    }
    widgets
}
