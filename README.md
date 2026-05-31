[English](./README.md) | [한국어](./README.ko.md)

# kid

A lightweight, single-player fork of [Zed](https://github.com/zed-industries/zed) focused on
fast local editing with **terminal-based AI** (Claude Code, Codex CLI) and a built-in
**browser side panel**.

kid strips Zed's in-editor AI, collaboration, and telemetry stacks down to a lean editor core,
and adds a native browser panel for live-previewing a local app right next to your code.

## Differences from upstream Zed

**Removed**

- **In-editor AI / LLM** — the agent panel, inline edit prediction, Copilot, and every model
  provider (Anthropic, OpenAI, Google, Ollama, Bedrock, Mistral, …). Run `claude` or `codex`
  in the integrated terminal instead.
- **Collaboration / multiplayer** — channels, calls, screen-sharing, the collab panel, the
  LiveKit client, and the collab server.
- **Telemetry** — all usage-analytics event collection (no usage data is gathered or sent).
- **Vim/Helix modal editing**, the in-editor **extensions browser UI**, and the benchmark crates.

**Kept** — the editor, language & LSP support, project/file management, Git & GitHub integration,
terminal, tasks, debugger, the **extension host** (so language servers and extensions still load),
file finder, and the REPL.

**Added**

- **Browser side panel** — a native WebView (via [wry](https://github.com/tauri-apps/wry)) docked
  beside the editor. See [Browser panel](#browser-panel).

**Defaults** — only the **Ayu Dark** theme and the **VS Code** base keymap are bundled.

## Building (macOS, Apple Silicon)

Prerequisites:

- **Rust** — the repo pins a toolchain via `rust-toolchain.toml`.
- **CMake** — `brew install cmake` (required to build the wasmtime/extension runtime).
- **Full Xcode + the Metal toolchain** — Command Line Tools alone are *not* enough, because gpui
  compiles Metal shaders at build time:

  ```sh
  sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
  sudo xcodebuild -license accept
  # On macOS 15+/Xcode 16.3+ the Metal compiler is a separate component:
  xcodebuild -downloadComponent MetalToolchain
  xcrun metal --version   # should print a version
  ```

Build and run:

```sh
cargo run --release -p zed
```

## Browser panel

- **Toggle:** `Cmd+Shift+B` opens/focuses the Browser panel in the right dock.
- **Default URL:** `http://localhost:3000`. Type a URL in the bar and press <kbd>Enter</kbd> to
  navigate; click the reload button to refresh.
- The panel hosts a **native WebView**, positioned to match the panel's bounds and hidden when the
  panel is closed, so it stays out of the editor's own rendering.

## Licensing & attribution

kid is a fork of **[zed-industries/zed](https://github.com/zed-industries/zed)** and remains
licensed under **GPL-3.0-or-later**, with Apache-2.0 components where marked upstream. The original
license texts are preserved verbatim in [LICENSE-GPL](./LICENSE-GPL) and
[LICENSE-APACHE](./LICENSE-APACHE). Per the GPL, the complete corresponding source is this
repository. All credit for the underlying editor goes to Zed Industries and the Zed contributors.
