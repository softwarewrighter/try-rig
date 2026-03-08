use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct CalcArgs {
    pub operation: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("Calculator error: {0}")]
pub struct CalcError(String);

#[derive(Deserialize, Serialize)]
pub struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculator";
    type Error = CalcError;
    type Args = CalcArgs;
    type Output = f64;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "calculator".to_string(),
            description: "Perform arithmetic operations: add, subtract, multiply, divide"
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "description": "The operation to perform: add, subtract, multiply, or divide",
                        "enum": ["add", "subtract", "multiply", "divide"]
                    },
                    "x": {
                        "type": "number",
                        "description": "The first operand"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second operand"
                    }
                },
                "required": ["operation", "x", "y"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        match args.operation.as_str() {
            "add" => Ok(args.x + args.y),
            "subtract" => Ok(args.x - args.y),
            "multiply" => Ok(args.x * args.y),
            "divide" => {
                if args.y == 0.0 {
                    Err(CalcError("Division by zero".to_string()))
                } else {
                    Ok(args.x / args.y)
                }
            }
            op => Err(CalcError(format!("Unknown operation: {op}"))),
        }
    }
}
