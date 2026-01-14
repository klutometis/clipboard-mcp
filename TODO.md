# TODO

## MVP: Clipboard Image Transcription MCP Server

### Immediate Goal
Transcribe screenshots of Roblox Studio logs (F9 console) from clipboard.

### Tech Stack Decision
- **Language**: TBD - TypeScript, Go, or Rust?
- **LLM Provider**: Gemini (REST API, no SDK needed)
- **Clipboard Access**: X11 (`xclip` or similar)

### MVP Implementation
- [ ] Choose language/SDK (TypeScript recommended)
- [ ] Set up MCP server skeleton
- [ ] Implement clipboard image reading (X11)
- [ ] Implement Gemini API integration (REST)
- [ ] Create `transcribe_clipboard_image` tool
  - Parameters: none for MVP (hardcode Gemini)
  - Returns: Plain text transcription
- [ ] Test with Roblox Studio screenshot
- [ ] Basic error handling (no image in clipboard, API failures)

### Future Enhancements (post-MVP)
- [ ] Add `transcribe_image` tool (file path support)
- [ ] Add `describe_clipboard_image` tool (non-text images)
- [ ] Support multiple providers (Claude, GPT)
- [ ] Custom prompt parameter
- [ ] Better error messages

### Design Notes
- Keep it stupid simple for MVP
- MCP server sends image to Gemini, returns only text (no base64 bloat in ACP)
- Focus: solve the Roblox screenshot pain point first
- Extensibility: easy to add more tools/providers later

### Open Questions
- Which language? (need to decide before scaffolding)
- Gemini API key configuration? (env var probably fine for MVP)
