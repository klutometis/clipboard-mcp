use base64::Engine;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
    transport::stdio,
    ErrorData as McpError, ServiceExt,
};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DescribeImageRequest {
    /// Optional: Specific aspect to focus on (e.g., 'What UI framework is this?', 'Describe the error message', 'What colors are used?')
    pub focus: Option<String>,
}

#[derive(Clone)]
pub struct ClipboardServer {
    gemini_api_key: String,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ClipboardServer {
    fn new(gemini_api_key: String) -> Self {
        Self {
            gemini_api_key,
            tool_router: Self::tool_router(),
        }
    }

    /// Read image from X11 clipboard using xclip
    fn read_clipboard_image(&self) -> Result<Vec<u8>, String> {
        let output = Command::new("xclip")
            .args(["-selection", "clipboard", "-t", "image/png", "-o"])
            .output()
            .map_err(|e| format!("Failed to execute xclip: {}", e))?;

        if !output.status.success() {
            return Err(
                "Failed to read image from clipboard. Is there an image in the clipboard?"
                    .to_string(),
            );
        }

        Ok(output.stdout)
    }

    /// Send image to Gemini API with a custom prompt
    async fn analyze_with_gemini(&self, image_data: Vec<u8>, prompt: String) -> Result<String, String> {
        let base64_image = base64::prelude::BASE64_STANDARD.encode(&image_data);

        let client = reqwest::Client::new();
        let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";

        #[derive(serde::Serialize)]
        struct Request {
            contents: Vec<Content>,
        }

        #[derive(serde::Serialize)]
        struct Content {
            parts: Vec<Part>,
        }

        #[derive(serde::Serialize)]
        #[serde(untagged)]
        enum Part {
            Text { text: String },
            InlineData { inline_data: InlineData },
        }

        #[derive(serde::Serialize)]
        struct InlineData {
            mime_type: String,
            data: String,
        }

        #[derive(serde::Deserialize)]
        struct Response {
            candidates: Vec<Candidate>,
        }

        #[derive(serde::Deserialize)]
        struct Candidate {
            content: ResponseContent,
        }

        #[derive(serde::Deserialize)]
        struct ResponseContent {
            parts: Vec<ResponsePart>,
        }

        #[derive(serde::Deserialize)]
        struct ResponsePart {
            text: String,
        }

        let request_body = Request {
            contents: vec![Content {
                parts: vec![
                    Part::InlineData {
                        inline_data: InlineData {
                            mime_type: "image/png".to_string(),
                            data: base64_image,
                        },
                    },
                    Part::Text {
                        text: prompt,
                    },
                ],
            }],
        };

        let response = client
            .post(url)
            .header("x-goog-api-key", &self.gemini_api_key)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Failed to send request to Gemini: {}", e))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Gemini API error: {}", error_text));
        }

        let gemini_response: Response = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

        let text = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_else(|| "[No response from Gemini]".to_string());

        Ok(text)
    }

    #[tool(description = "Transcribe text from an image in the clipboard using Gemini AI. The image must be in the X11 clipboard (copied with Ctrl+C or similar). Returns the transcribed text.")]
    async fn transcribe_clipboard_image(&self) -> Result<CallToolResult, McpError> {
        // Read image from clipboard
        let image_data = match self.read_clipboard_image() {
            Ok(data) => data,
            Err(e) => {
                return Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                    format!("Error reading clipboard: {}", e),
                )]));
            }
        };

        // Send to Gemini for transcription
        let prompt = "Transcribe all text from this image exactly as it appears. If there are multiple lines, preserve the line breaks. If there is no text, respond with '[No text found]'.".to_string();
        let transcribed_text = match self.analyze_with_gemini(image_data, prompt).await {
            Ok(text) => text,
            Err(e) => {
                return Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                    format!("Error transcribing with Gemini: {}", e),
                )]));
            }
        };

        Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            transcribed_text,
        )]))
    }

    #[tool(description = "Describe the contents of an image in the clipboard using Gemini AI. Optionally provide a focus query to get specific information about the image.")]
    async fn describe_clipboard_image(
        &self,
        params: Parameters<DescribeImageRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Read image from clipboard
        let image_data = match self.read_clipboard_image() {
            Ok(data) => data,
            Err(e) => {
                return Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                    format!("Error reading clipboard: {}", e),
                )]));
            }
        };

        // Build prompt based on whether focus is provided
        let prompt = match &params.0.focus {
            Some(query) if !query.is_empty() => format!(
                "Describe this image, focusing specifically on: {}\n\nProvide a clear, detailed response.",
                query
            ),
            _ => "Describe this image in detail. Include what you see, the layout, colors, key elements, and any notable features. If there is text, mention it but don't transcribe it fully unless it's critical to understanding the image.".to_string(),
        };

        // Send to Gemini for description
        let description = match self.analyze_with_gemini(image_data, prompt).await {
            Ok(text) => text,
            Err(e) => {
                return Ok(CallToolResult::error(vec![rmcp::model::Content::text(
                    format!("Error describing with Gemini: {}", e),
                )]));
            }
        };

        Ok(CallToolResult::success(vec![rmcp::model::Content::text(
            description,
        )]))
    }
}

// Implement the server handler
#[tool_handler]
impl rmcp::ServerHandler for ClipboardServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Clipboard image analysis server. Use transcribe_clipboard_image to extract text, or describe_clipboard_image to get visual descriptions of images in your clipboard.".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Gemini API key from environment
    let gemini_api_key =
        std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable must be set");

    let server = ClipboardServer::new(gemini_api_key);

    // Create and run the server with STDIO transport
    let service = server.serve(stdio()).await.inspect_err(|e| {
        eprintln!("Error starting server: {}", e);
    })?;

    service.waiting().await?;

    Ok(())
}
