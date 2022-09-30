use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rocket::serde::json;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

mod configuration;
use configuration::Configuration;

#[derive(RustEmbed)]
#[folder = "src/configuration_frontend/dist"]
struct Asset;

#[post("/config", format = "json", data = "<config>")]
async fn save_config(config: json::Json<Configuration>) -> json::Value {
    // let db: sled::Db = sled::open("wg_display.db").unwrap();
    // println!("Saving config text: {}", config.text);
    // db.insert("config_text", config.text.as_bytes()).unwrap();
    json::json!({ "status": "ok" })
}

#[get("/config")]
fn get_config() -> Option<json::Value> {
    let db: sled::Db = sled::open("wg_display.db").unwrap();
    let config_text = db
        .get("config_text")
        .unwrap()
        .map(|v| String::from_utf8(v.to_vec()).unwrap_or_else(|_| String::from("")));

    Some(json::json!(Configuration {
        example_value: config_text.unwrap()
    }))
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
