# clipboard-mcp

An MCP (Model Context Protocol) server that transcribes text from images in your X11 clipboard using Gemini AI.

## Problem Statement

Reading images via ACP in agent-shell is broken - it fills up the context window with base64 garbage. This server provides a clean solution: grab an image from your clipboard (e.g., Roblox Studio screenshots), send it to Gemini for transcription, and return only the extracted text.

## Requirements

- Rust toolchain (install via [rustup](https://rustup.rs/))
- `xclip` for X11 clipboard access:
  ```bash
  sudo apt install xclip  # Debian/Ubuntu
  ```
- Gemini API key (get one from [Google AI Studio](https://makersuite.google.com/app/apikey))

## Installation

### Install from source

1. Clone and install:
   ```bash
   git clone https://github.com/klutometis/clipboard-mcp.git
   cd clipboard-mcp
   cargo install --path .
   ```

   This installs the binary to `~/.cargo/bin/clipboard-mcp`.

2. Set your Gemini API key:
   ```bash
   export GEMINI_API_KEY="your-api-key-here"
   ```

## Usage

### Testing with MCP Inspector

The easiest way to test your server is with the official MCP Inspector:

```bash
# Make sure xclip is installed
sudo apt install xclip  # if not already installed

# Copy an image to your clipboard (screenshot or Ctrl+C)

# Run the inspector
GEMINI_API_KEY="your-key" npx @modelcontextprotocol/inspector \
  /path/to/clipboard-mcp/target/release/clipboard-mcp
```

This will:
1. Start a web UI at http://localhost:6274
2. Connect to your MCP server
3. Show available tools (you should see `transcribe_clipboard_image`)
4. Let you test the tool by clicking it

### Running the Server Standalone

The server uses stdio transport (standard input/output):

```bash
GEMINI_API_KEY="your-key" cargo run --release
```

### Configuring in agent-shell / ACP

Add to your MCP configuration (e.g., `~/.mcp.json`):

```json
{
  "mcpServers": {
    "clipboard": {
      "command": "clipboard-mcp",
      "env": {
        "GEMINI_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

Or with full path if `~/.cargo/bin` is not in your PATH:

```json
{
  "mcpServers": {
    "clipboard": {
      "command": "/home/yourusername/.cargo/bin/clipboard-mcp",
      "env": {
        "GEMINI_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

### Using the Tool

Once configured, you can use the `transcribe_clipboard_image` tool in your agent session:

1. Copy an image to your clipboard (Ctrl+C in most apps, or screenshot tools)
2. In your agent session: "Please transcribe the image in my clipboard"
3. The agent will call the tool and return the extracted text

## Example Use Case

**Problem:** Roblox Studio F9 console doesn't support copy-paste for logs.

**Solution:**
1. Take a screenshot of the console (or copy the game window)
2. Ask your agent: "Transcribe the logs from my clipboard"
3. Get clean text output without base64 bloat

## How It Works

1. Reads PNG image data from X11 clipboard via `xclip`
2. Base64 encodes the image
3. Sends to Gemini 3 Flash API with transcription prompt
4. Returns extracted text to the MCP client

## Limitations

- **X11 only**: Uses `xclip`, so Linux/X11 required (not Wayland, macOS, or Windows)
- **PNG format**: Currently hardcoded to read PNG images from clipboard
- **Single tool**: MVP focuses on transcription only

## Future Enhancements

- [ ] Add `describe_clipboard_image` for non-text images
- [ ] Support file path input (`transcribe_image` tool)
- [ ] Support multiple providers (Claude, GPT)
- [ ] Support Wayland clipboard (`wl-clipboard`)
- [ ] Support multiple image formats (JPEG, etc.)
- [ ] Custom prompt parameter

## Development

Run in debug mode:
```bash
GEMINI_API_KEY="your-key" cargo run
```

Check the code:
```bash
cargo clippy
cargo fmt
```

## License

MIT (or whatever you prefer)
