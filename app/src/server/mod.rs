use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rocket::serde::json;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

use common::models::Configuration;

use crate::shared::persistence::Persistence;

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;

#[post("/config", format = "json", data = "<config>")]
async fn save_config(config: json::Json<Configuration>) {
    Persistence::save_config(config.into_inner());
}

#[get("/config")]
fn get_config() -> Option<json::Value> {
    Some(json::json!(Persistence::get_config()))
}

#[get("/")]
async fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

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

pub fn serve_dashboard() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, dist, save_config, get_config])
}
