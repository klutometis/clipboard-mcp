# TODO

## Current Status: MVP Complete + Description Tool

### Completed ✓
- [x] Choose language/SDK - **Rust with rmcp**
- [x] Set up MCP server skeleton
- [x] Implement clipboard image reading (X11 via xclip)
- [x] Implement Gemini API integration (REST)
- [x] Create `transcribe_clipboard_image` tool
- [x] Test with Roblox Studio screenshot
- [x] Basic error handling (no image in clipboard, API failures)
- [x] Add `describe_clipboard_image` tool with optional focus parameter

### In Progress
None

### Future Enhancements

#### High Priority
- [ ] Add file path support
  - [ ] `transcribe_image` tool (accepts file path)
  - [ ] `describe_image` tool (accepts file path + optional focus)
- [ ] Support multiple image formats (JPEG, GIF, WebP)
  - Currently hardcoded to PNG
  - Need to detect format or try multiple mime types

#### Medium Priority
- [ ] Support Wayland clipboard (`wl-clipboard`)
  - Currently X11-only via xclip
- [ ] Add custom model selection parameter
  - Allow choosing between Gemini models (Flash, Pro, etc.)
- [ ] Better error messages and validation
  - Detect empty clipboard before calling Gemini
  - Provide helpful error messages

#### Low Priority
- [ ] Support multiple LLM providers
  - Claude (Anthropic)
  - GPT-4 Vision (OpenAI)
  - Make provider configurable per-request or via env var

#### Nice to Have
- [ ] Caching support (avoid re-analyzing same image)
- [ ] Batch processing (multiple images at once)
- [ ] Custom prompts as parameters (advanced users)

### Design Decisions Made

1. **Separate tools** (transcribe vs describe) rather than combined
   - Different prompts for different tasks
   - Clear semantics and failure modes
   - Easier to optimize individually

2. **Optional focus parameter** via `Parameters<DescribeImageRequest>` struct
   - `Option<String>` in struct, not function parameter
   - Workaround for rmcp 0.12 limitations with optional params

3. **Rust + rmcp SDK** over TypeScript
   - Better performance
   - Strong typing
   - Native binary (no Node.js runtime needed)

### Open Questions
- Should we add streaming support for long transcriptions?
- How to handle very large images (size limits)?
- Should we add OCR fallback for low-quality images?
