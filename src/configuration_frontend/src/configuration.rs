use serde::{Deserialize, Serialize};

// TODO: share this config with the frontend
#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub example_value: String,
}
