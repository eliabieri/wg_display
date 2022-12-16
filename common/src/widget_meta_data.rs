#[derive(PartialEq, Clone)]
pub enum WidgetMetaData {
    Cafete,
    Aare,
    Today,
    Bernaqua,
    PublicTransport,
}

impl WidgetMetaData {
    pub fn name(&self) -> &'static str {
        match self {
            WidgetMetaData::Cafete => "Cafete",
            WidgetMetaData::Aare => "Aare",
            WidgetMetaData::Today => "Today",
            WidgetMetaData::Bernaqua => "Bernaqua",
            WidgetMetaData::PublicTransport => "Next departure",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            WidgetMetaData::Cafete => "Events happening at the Cafete Club in Bern",
            WidgetMetaData::Aare => "The temperature of the Aare river in Bern",
            WidgetMetaData::Today => "The current date and time",
            WidgetMetaData::Bernaqua => "Occupancy of the Bernaqua facilities",
            WidgetMetaData::PublicTransport => "Next public transport departures",
        }
    }
}
