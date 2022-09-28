use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rust_embed::RustEmbed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "src/configuration_frontend/dist"]
struct Asset;

#[get("/")]
fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("yew/index.html")?;
    Some(RawHtml(asset.data))
}

#[get("/<file..>")]
fn dist(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = if file.display().to_string().ends_with(".css") {
        format!("css/{}", file.display())
    } else {
        format!("yew/{}", file.display())
    };

    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

pub fn serve_dashboard() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, dist])
}
