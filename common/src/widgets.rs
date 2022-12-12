pub enum WidgetName {
    Cafete,
    Aare,
    Time,
    Bernaqua,
    PublicTransport,
}

impl WidgetName {
    pub fn as_str(&self) -> &'static str {
        match self {
            WidgetName::Cafete => "Cafete",
            WidgetName::Aare => "Aare",
            WidgetName::Time => "Time",
            WidgetName::Bernaqua => "Bernaqua",
            WidgetName::PublicTransport => "Next departure",
        }
    }
}
