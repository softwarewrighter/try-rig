use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct WeatherArgs {
    pub city: String,
}

#[derive(Debug, Serialize)]
pub struct WeatherInfo {
    pub city: String,
    pub temperature_f: f64,
    pub condition: String,
    pub humidity: u8,
}

#[derive(Debug, thiserror::Error)]
#[error("Weather error: {0}")]
pub struct WeatherError(String);

#[derive(Deserialize, Serialize)]
pub struct WeatherLookup;

impl WeatherLookup {
    fn simulated_data() -> HashMap<String, (f64, &'static str, u8)> {
        HashMap::from([
            ("new york".into(), (45.0, "Cloudy", 65)),
            ("london".into(), (50.0, "Rainy", 80)),
            ("tokyo".into(), (60.0, "Sunny", 40)),
            ("paris".into(), (55.0, "Partly Cloudy", 55)),
            ("sydney".into(), (75.0, "Sunny", 35)),
            ("san francisco".into(), (58.0, "Foggy", 70)),
            ("seattle".into(), (48.0, "Rainy", 85)),
            ("chicago".into(), (38.0, "Windy", 50)),
        ])
    }
}

impl Tool for WeatherLookup {
    const NAME: &'static str = "get_weather";
    type Error = WeatherError;
    type Args = WeatherArgs;
    type Output = WeatherInfo;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_weather".to_string(),
            description: "Get current weather information for a city".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "The city name to look up weather for"
                    }
                },
                "required": ["city"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let city_lower = args.city.to_lowercase();
        let data = Self::simulated_data();

        match data.get(city_lower.as_str()) {
            Some(&(temp, condition, humidity)) => Ok(WeatherInfo {
                city: args.city,
                temperature_f: temp,
                condition: condition.to_string(),
                humidity,
            }),
            None => Err(WeatherError(format!(
                "No weather data available for '{}'. Available cities: {}",
                args.city,
                data.keys().cloned().collect::<Vec<_>>().join(", ")
            ))),
        }
    }
}
