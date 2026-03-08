use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

#[derive(Deserialize)]
pub struct SearchArgs {
    pub directory: String,
    pub pattern: String,
}

#[derive(Debug, thiserror::Error)]
#[error("File search error: {0}")]
pub struct FileSearchError(String);

#[derive(Deserialize, Serialize)]
pub struct FileSearch;

impl Tool for FileSearch {
    const NAME: &'static str = "search_files";
    type Error = FileSearchError;
    type Args = SearchArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "search_files".to_string(),
            description: "Search for files matching a glob pattern in a directory".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "directory": {
                        "type": "string",
                        "description": "The directory path to search in"
                    },
                    "pattern": {
                        "type": "string",
                        "description": "Glob pattern to match files (e.g. '*.rs', '**/*.toml')"
                    }
                },
                "required": ["directory", "pattern"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let dir = Path::new(&args.directory);
        if !dir.exists() {
            return Err(FileSearchError(format!(
                "Directory '{}' does not exist",
                args.directory
            )));
        }

        let full_pattern = format!("{}/{}", args.directory, args.pattern);
        let paths: Vec<String> = glob::glob(&full_pattern)
            .map_err(|e| FileSearchError(format!("Invalid pattern: {e}")))?
            .filter_map(|entry| entry.ok())
            .map(|path| path.display().to_string())
            .collect();

        Ok(paths)
    }
}
