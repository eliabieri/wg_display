//! Serves the frontend files and provides an API to fetch and modify the configuration.
use rocket::config::Config;
use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rocket::serde::json;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

use common::models::SystemConfiguration;

use crate::shared::persistence::Persistence;

/// Contains the frontend files
/// They are embedded using the [RustEmbed](https://crates.io/crates/rust-embed) crate
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;

/// Saves the system configuration
#[post("/config", format = "json", data = "<config>")]
async fn save_config(config: json::Json<SystemConfiguration>) {
    Persistence::save_config(config.into_inner());
}

/// Returns the system configuration
#[get("/config")]
fn get_config() -> Option<json::Value> {
    Some(json::json!(Persistence::get_config()))
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
        .mount("/", routes![index, dist, save_config, get_config])
        .launch()
        .await?;
    Ok(())
}
