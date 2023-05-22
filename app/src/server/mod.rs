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

use common::models::{InstallAction, SystemConfiguration, WidgetStoreItem};

use crate::shared::persistence::Persistence;
use crate::shared::widget_manager::WidgetManager;
use crate::widgets::running::runtime::Runtime;
use crate::widgets::store::widget_store::WidgetStore;

/// Contains the frontend files
/// They are embedded using the [RustEmbed](https://crates.io/crates/rust-embed) crate
#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
struct Asset;

/// Returns the list of widgets available in the store
#[get("/get_store_items")]
async fn get_store_items() -> Result<json::Value, Custom<String>> {
    let mut store = WidgetStore::new();
    let res = store.fetch_from_store().await;
    if let Err(err) = res {
        return Err(Custom(
            rocket::http::Status::InternalServerError,
            format!("Could not update store: {}", err),
        ));
    }
    Ok(json::json!(store.get_items()))
}

/// Install a new widget
#[post("/install_widget", format = "json", data = "<action>")]
async fn install_widget(action: json::Json<InstallAction>) -> Result<(), Custom<String>> {
    let mut description = "No description".to_string();
    let download_url = match action {
        json::Json(InstallAction::FromUrl(url)) => url,
        json::Json(InstallAction::FromStoreItemName(name)) => {
            let mut store = WidgetStore::new();
            let res = store.fetch_from_store().await;
            if let Err(err) = res {
                return Err(Custom(
                    rocket::http::Status::InternalServerError,
                    format!("Could not update store: {}", err),
                ));
            }
            let item: &WidgetStoreItem = store
                .get_items()
                .iter()
                .find(|item: &&WidgetStoreItem| item.name == name)
                .unwrap();
            description = item.description.clone();
            item.get_download_url()
        }
    };
    log::info!("Installing widget from URL {}", download_url);
    let result = WidgetManager::install_widget(download_url.as_str(), description.as_str()).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => {
            let err = format!(
                "Could not install widget from URL {}: {}",
                download_url,
                err.root_cause()
            );
            log::error!("{}", err);
            Err(Custom(rocket::http::Status::InternalServerError, err))
        }
    }
}

/// Deinstall a widget
#[get("/deinstall_widget/<widget_name>")]
async fn deinstall_widget(widget_name: &str) -> Result<(), Custom<String>> {
    let result = WidgetManager::deinstall_widget(widget_name).await;
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(Custom(
            rocket::http::Status::InternalServerError,
            format!("Could not deinstall widget: {}", err),
        )),
    }
}

/// Returns the configuration schema of a widget
#[get("/config_schema/<widget_name>")]
fn get_config_schema(widget_name: &str) -> Option<String> {
    let mut runtime = Runtime::new();
    let component_binary = WidgetManager::get_widget(widget_name);
    let Ok(component_binary) = component_binary else {
        println!("Could not load WASM component");
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
#[post("/system_config", format = "json", data = "<config>")]
async fn save_system_config(config: json::Json<SystemConfiguration>) {
    Persistence::save_system_config(config.into_inner());
}

/// Saves a widget config
#[post("/widget_config/<widget_name>", data = "<config>")]
async fn save_widget_config(widget_name: &str, config: String) {
    let mut system_config = Persistence::get_system_config().unwrap();
    system_config.widgets.iter_mut().for_each(|widget| {
        if widget.name == widget_name {
            widget.json_config = config.clone();
        }
    });
    Persistence::save_system_config(system_config);
}

/// Returns the system configuration
#[get("/system_config")]
fn get_system_config() -> Option<json::Value> {
    Some(json::json!(Persistence::get_system_config()))
}

/// Serves index.html
#[get("/")]
async fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

/// Serves a configuration page for a widget
#[get("/widget_configuration/<widget_name>")]
async fn widget_configuration(widget_name: &str) -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("assets/html/widget_config.html")?;
    let html = String::from_utf8(asset.data.to_vec()).unwrap();
    let html = html.replace("{{WIDGET_NAME}}", widget_name);
    let data = Cow::from(html.into_bytes());
    Some(RawHtml(data))
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
                widget_configuration,
                dist,
                save_system_config,
                save_widget_config,
                get_system_config,
                get_config_schema,
                install_widget,
                deinstall_widget,
                get_store_items
            ],
        )
        .launch()
        .await?;
    Ok(())
}
