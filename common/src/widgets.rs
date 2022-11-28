pub enum WidgetName {
    Cafete,
    Aare,
    Time,
    Bernaqua,
}

impl WidgetName {
    pub fn as_str(&self) -> &'static str {
        match self {
            WidgetName::Cafete => "Cafete",
            WidgetName::Aare => "Aare",
            WidgetName::Time => "Time",
            WidgetName::Bernaqua => "Bernaqua occupancy",
        }
    }
}
