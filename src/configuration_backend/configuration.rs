use serde::{Deserialize, Serialize};

// TODO: share this config with the frontend
#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub example_value: String,
}
