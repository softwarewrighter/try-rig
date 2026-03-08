use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct StringArgs {
    pub operation: String,
    pub text: String,
    pub param: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("String tool error: {0}")]
pub struct StringError(String);

#[derive(Debug, Serialize)]
pub struct StringResult {
    pub result: String,
    pub length: usize,
}

#[derive(Deserialize, Serialize)]
pub struct StringTool;

impl Tool for StringTool {
    const NAME: &'static str = "string_tool";
    type Error = StringError;
    type Args = StringArgs;
    type Output = StringResult;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "string_tool".to_string(),
            description: "Perform string operations: uppercase, lowercase, reverse, count_words, replace, trim"
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "description": "The string operation to perform",
                        "enum": ["uppercase", "lowercase", "reverse", "count_words", "replace", "trim"]
                    },
                    "text": {
                        "type": "string",
                        "description": "The input text to operate on"
                    },
                    "param": {
                        "type": "string",
                        "description": "Optional parameter. For 'replace': use 'old:new' format to specify the substitution."
                    }
                },
                "required": ["operation", "text"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = match args.operation.as_str() {
            "uppercase" => args.text.to_uppercase(),
            "lowercase" => args.text.to_lowercase(),
            "reverse" => args.text.chars().rev().collect(),
            "count_words" => {
                let count = args.text.split_whitespace().count();
                return Ok(StringResult {
                    result: count.to_string(),
                    length: args.text.len(),
                });
            }
            "replace" => {
                let param = args
                    .param
                    .ok_or_else(|| StringError("replace requires param in 'old:new' format".into()))?;
                let parts: Vec<&str> = param.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(StringError("param must be in 'old:new' format".into()));
                }
                args.text.replace(parts[0], parts[1])
            }
            "trim" => args.text.trim().to_string(),
            op => return Err(StringError(format!("Unknown operation: {op}"))),
        };
        let length = result.len();
        Ok(StringResult { result, length })
    }
}
