//! Serves the frontend files and provides an API to fetch and modify the configuration.
use rocket::config::Config;
use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rocket::response::status::Custom;
use rocket::serde::json;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

use common::models::{InstallationData, SystemConfiguration};

use crate::shared::persistence::Persistence;
use crate::shared::widget_manager::WidgetManager;
use crate::widgets::running::runtime::Runtime;

/// Contains the frontend files
/// They are embedded using the [RustEmbed](https://crates.io/crates/rust-embed) crate
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;

/// Install a new widget
#[post("/install_widget", format = "json", data = "<installation_data>")]
async fn install_widget(
    installation_data: json::Json<InstallationData>,
) -> Result<(), Custom<String>> {
    let InstallationData { download_url } = installation_data.into_inner();
    let result = WidgetManager::install_widget(download_url.as_str()).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(Custom(
            rocket::http::Status::InternalServerError,
            format!("Could not install widget: {}", err),
        )),
    }
}

/// Returns the configuration schema of a widget
#[get("/config_schema/<widget_name>")]
fn get_config_schema(widget_name: &str) -> Option<String> {
    let mut runtime = Runtime::new();
    let component_binary = WidgetManager::get_widget(widget_name);
    let Ok(component_binary) = component_binary else {
        println!("Could not load WASM module");
        return None;
    };

    let widget = runtime.instantiate_widget(component_binary.as_slice());
    let Ok(widget) = widget else {
        println!("Could not instantiate widget");
        return None;
    };

    let schema = runtime.get_config_schema(&widget);
    let Ok(schema) = schema else {
        println!("Could not get config schema");
        return None;
    };
    Some(schema)
}

/// Saves the system configuration
#[post("/config", format = "json", data = "<config>")]
async fn save_config(config: json::Json<SystemConfiguration>) {
    Persistence::save_config(config.into_inner());
}

/// Returns the system configuration
#[get("/config")]
fn get_config() -> Option<json::Value> {
    Some(json::json!(Persistence::get_system_config()))
}

/// Serves index.html
#[get("/")]
async fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

//// Serves the frontend files (WASM, JS, HTML, CSS, etc.)
#[get("/<file..>")]
fn dist(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();

    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

/// Starts the server to serve the frontend and the API to fetch and modify the configuration.
pub async fn serve_dashboard() -> Result<(), rocket::Error> {
    // Make dashboard accessible from outside
    let config = Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("log_level", "off"));
    let _rocket = rocket::custom(config)
        .mount(
            "/",
            routes![
                index,
                dist,
                save_config,
                get_config,
                get_config_schema,
                install_widget
            ],
        )
        .launch()
        .await?;
    Ok(())
}
