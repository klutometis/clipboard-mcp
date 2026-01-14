# NOTES

## Project Context

This MCP server solves a specific problem: reading images via ACP in agent-shell fills up the context window with base64 garbage. Instead, we grab the image from the clipboard, send it to Gemini AI, and return only the extracted text or description.

## Implementation Journey

### Initial MVP (Transcription Only)
- Started with simple text transcription from clipboard images
- Used Rust + rmcp SDK for performance and type safety
- Chose Gemini 3 Flash for fast, cost-effective processing
- Hardcoded to PNG format via xclip for MVP speed

### Adding Description Tool

**Problem**: Need to describe non-text images (diagrams, UI screenshots, etc.), not just transcribe text.

**Design Question**: Combined tool or separate tools?

**Decision**: Separate tools (`transcribe_clipboard_image` vs `describe_clipboard_image`)

**Rationale**:
1. **Prompt Optimization**: Transcription needs "extract ALL text exactly", description needs "describe what you see"
2. **Clear Semantics**: User/agent knows exactly what they're getting
3. **Failure Modes**: `"[No text found]"` is useful for transcribe, not for describe
4. **Future Flexibility**: Can optimize/cache differently per tool

### Optional Parameters Challenge

**Problem**: How to add an optional `focus` parameter to `describe_clipboard_image`?

**First Attempt**: Use `Option<String>` directly as function parameter
```rust
async fn describe_clipboard_image(&self, focus: Option<String>)
```
Result: **Compilation error** - rmcp 0.12's `#[tool]` macro doesn't support optional params

**Second Attempt**: Use `#[serde(default)]` attribute
```rust
async fn describe_clipboard_image(&self, #[serde(default)] focus: Option<String>)
```
Result: **Compilation error** - `#[serde]` not valid on function parameters

**Third Attempt**: Use empty string to represent "no focus"
```rust
async fn describe_clipboard_image(&self, focus: String)
```
Result: **Still fails** - same trait bound error persists

**Solution**: Use `Parameters<StructType>` wrapper pattern
```rust
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DescribeImageRequest {
    pub focus: Option<String>,
}

async fn describe_clipboard_image(&self, params: Parameters<DescribeImageRequest>)
```
Result: **Success!** This is the official pattern from rust-sdk examples.

**Key Learning**: The `#[tool]` macro requires structs with `JsonSchema` derive for complex parameters. Optional fields work fine in structs, just not as direct function parameters.

## Technical Details

### Gemini API Integration
- Using REST API directly (no SDK needed)
- Model: `gemini-3-flash-preview` (fast and cheap)
- Input: Base64-encoded PNG image + text prompt
- Output: Extracted text or description

### Prompt Engineering

**Transcription Prompt**:
```
Transcribe all text from this image exactly as it appears. 
If there are multiple lines, preserve the line breaks. 
If there is no text, respond with '[No text found]'.
```

**Description Prompt (General)**:
```
Describe this image in detail. Include what you see, the layout, 
colors, key elements, and any notable features. If there is text, 
mention it but don't transcribe it fully unless it's critical to 
understanding the image.
```

**Description Prompt (Focused)**:
```
Describe this image, focusing specifically on: {user_query}

Provide a clear, detailed response.
```

## Architecture Patterns

### Code Reuse
Both tools share the same underlying infrastructure:
- `read_clipboard_image()` - X11 clipboard access
- `analyze_with_gemini(image_data, prompt)` - Generic Gemini API call
- Only difference is the prompt construction

### Error Handling
- Clipboard read failures return user-friendly error via `CallToolResult::error()`
- Gemini API failures include error text from response body
- No panics - all errors gracefully returned to client

### Extensibility Points
1. **New Tools**: Easy to add file-path variants by reusing `analyze_with_gemini()`
2. **New Providers**: Abstract `analyze_with_gemini()` into trait for multi-provider support
3. **New Formats**: Currently hardcoded PNG, but could detect/try multiple formats

## Lessons Learned

1. **Check Examples First**: The rust-sdk examples showed the `Parameters<T>` pattern immediately
2. **Semantic Versioning Confusion**: 0.12 > 0.2, not the other way around (I got this wrong initially)
3. **rmcp Limitations**: Optional parameters aren't supported directly by `#[tool]` macro in 0.12
4. **Design Clarity**: Keeping tools separate was the right call - simpler to reason about and extend

## Future Considerations

### Multi-Format Support
Currently hardcoded to PNG. Potential approaches:
- Try multiple mime types in sequence (PNG, JPEG, WebP)
- Use file magic to detect format
- Accept format parameter

### Provider Abstraction
If adding Claude/GPT support:
```rust
trait VisionProvider {
    async fn analyze(&self, image: Vec<u8>, prompt: String) -> Result<String>;
}
```

### Wayland Support
Need to:
1. Detect X11 vs Wayland at runtime
2. Use `wl-paste` instead of `xclip` for Wayland
3. Keep X11 support for compatibility

### Performance Optimization
- Could cache analysis results by image hash
- Could batch multiple images in single request
- Could use streaming for long responses
