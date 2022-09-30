use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rocket::serde::json;
use rocket::State;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

use common::models::Configuration;

mod persistence;
use persistence::Persistence;

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;

#[post("/config", format = "json", data = "<config>")]
async fn save_config(persistence: &State<Persistence>, config: json::Json<Configuration>) {
    persistence.save_config(config.into_inner());
}

#[get("/config")]
fn get_config(persistence: &State<Persistence>) -> Option<json::Value> {
    Some(json::json!(persistence.get_config()))
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
    rocket::build()
        .mount("/", routes![index, dist, save_config, get_config])
        .manage(Persistence::new())
}
