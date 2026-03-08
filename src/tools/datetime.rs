use chrono::{Local, Utc};
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct DateTimeArgs {
    pub timezone: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("DateTime error: {0}")]
pub struct DateTimeError(String);

#[derive(Debug, Serialize)]
pub struct DateTimeInfo {
    pub datetime: String,
    pub timezone: String,
    pub unix_timestamp: i64,
}

#[derive(Deserialize, Serialize)]
pub struct DateTime;

impl Tool for DateTime {
    const NAME: &'static str = "get_datetime";
    type Error = DateTimeError;
    type Args = DateTimeArgs;
    type Output = DateTimeInfo;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_datetime".to_string(),
            description: "Get the current date and time. Optionally specify timezone (utc or local) and format."
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "timezone": {
                        "type": "string",
                        "description": "Timezone: 'utc' or 'local'. Defaults to 'local'.",
                        "enum": ["utc", "local"]
                    },
                    "format": {
                        "type": "string",
                        "description": "Optional strftime format string, e.g. '%Y-%m-%d %H:%M:%S'"
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tz = args.timezone.unwrap_or_else(|| "local".to_string());
        let fmt = args
            .format
            .unwrap_or_else(|| "%Y-%m-%d %H:%M:%S".to_string());

        match tz.as_str() {
            "utc" => {
                let now = Utc::now();
                Ok(DateTimeInfo {
                    datetime: now.format(&fmt).to_string(),
                    timezone: "UTC".to_string(),
                    unix_timestamp: now.timestamp(),
                })
            }
            _ => {
                let now = Local::now();
                Ok(DateTimeInfo {
                    datetime: now.format(&fmt).to_string(),
                    timezone: "Local".to_string(),
                    unix_timestamp: now.timestamp(),
                })
            }
        }
    }
}
