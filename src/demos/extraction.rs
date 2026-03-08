use anyhow::Result;
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ContactInfo {
    /// The person's full name
    pub name: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Phone number
    pub phone: Option<String>,
}

pub async fn run(model: &str, text: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;

    let extractor = client
        .extractor::<ContactInfo>(model)
        .preamble(
            "Extract contact information from the provided text. \
             Pull out the person's name, email address, and phone number if present.",
        )
        .build();

    let contact = extractor.extract(text).await?;
    Ok(serde_json::to_string_pretty(&contact)?)
}
