# CHANGELOG

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `describe_clipboard_image` tool for visual description of images
  - Optional `focus` parameter to guide the description (e.g., "What UI framework is this?")
  - Uses `Parameters<DescribeImageRequest>` struct pattern for optional parameters
  - Shares infrastructure with transcription tool via `analyze_with_gemini()`

### Changed
- Refactored `transcribe_with_gemini()` → `analyze_with_gemini()` to accept custom prompts
- Updated README with usage examples for both tools
- Updated server instructions to mention both tools

### Added (Dependencies)
- `schemars` 1.0 - JSON Schema support for tool parameters

## [0.1.0] - 2025-01-13

### Added
- Initial MVP release
- `transcribe_clipboard_image` tool for extracting text from clipboard images
- X11 clipboard support via `xclip`
- Gemini 3 Flash API integration
- Rust-based MCP server using rmcp SDK
- Support for PNG images from clipboard
- Basic error handling for clipboard and API failures
- Installation via `cargo install`
- README with usage instructions and examples

### Technical Details
- Uses stdio transport for MCP communication
- Gemini API key via `GEMINI_API_KEY` environment variable
- Base64 encoding for image transmission
- Optimized for low latency with Gemini Flash model

## Future Releases

See [TODO.md](TODO.md) for planned features and enhancements.
