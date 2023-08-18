use std::{
    fs::{self, File},
    io::Write,
};

use build_html::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct WidgetStoreItem {
    name: String,
    description: String,
    repository: String,
}

fn main() {
    let data = fs::read_to_string("../../widget_store.json").expect("Unable to read file");
    let widget_store_items: Vec<WidgetStoreItem> =
        serde_json::from_str(&data).expect("Could not parse widget_store JSON file");

    let mut table = Table::new();
    table.add_header_row(&["Name", "Description", "Repository"]);
    for item in widget_store_items {
        table.add_body_row(&[item.name, item.description, item.repository]);
    }

    let html: String = HtmlPage::new()
        .with_title("WG Display Widget Store")
        .with_table(table)
        .to_html_string();

    let file = File::create("widget_store_page.html").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(html.as_bytes()).unwrap();
}
