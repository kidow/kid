# kid — Crate Removal & Browser Panel Plan

Fork of `zed-industries/zed`. Goal: slim the build by removing in-editor AI,
collaboration, telemetry, benchmarks, vim, and extension UI; add a `wry`-based
browser side panel. Integrity first: each crate removed sequentially, `cargo
build` green between steps. `gpui` and its sub-crates are never modified.

Baseline: `cargo build -p zed` → exit 0 (verified before any change).

## 1. Corrected inventory (goal list vs. this fork)

The goal's list assumes upstream zed. This fork has diverged:

| Goal target | Status in fork | Action |
|---|---|---|
| anthropic, google_ai, ollama, open_ai | exist (providers) | remove (with `language_models*`) |
| supermaven | **absent** | skip |
| assistant | **absent** → became `agent`/`agent_ui` | remove the `agent*` stack instead |
| copilot | exists (+ `copilot_chat`, `copilot_ui`) | remove all three |
| agent, agent_skills, agent_settings | exist (+ `agent_ui`, `agent_servers`) | remove; `agent_settings` is special (see §3) |
| collab | exists (server) | remove + whole UI stack (see §4) |
| feedback | exists | remove (leaf, zed-only) |
| telemetry | exists, **deeply coupled** | force-remove LAST (see §3) |
| eval | **absent** → `eval_cli` + `eval_utils` | remove those |
| eval_cli | exists (leaf) | remove |
| editor_benchmarks, fs_benchmarks | exist (leaves) | remove |
| extensions_ui | exists (zed-only) | remove |
| vim | exists (clean leaf) | remove |

`welcome` is **not a crate** — it is `crates/workspace/src/welcome.rs` plus the
`onboarding` crate. Theme/keymap picker UI lives in
`crates/onboarding/src/base_keymap_picker.rs`.

## 2. Decisions (confirmed by owner)

- **telemetry** → force-remove (not neutralize). Done last, after the AI/collab
  removals delete most of its callers, shrinking the must-keep cleanup surface.
- **AI surface** → remove everything not needed to run claude/codex in the
  integrated terminal. No in-editor AI; no ACP external-agent panel. Terminal
  (`terminal_view`, kept) is the only AI entry point.
- **collab** → remove the whole collaboration stack, not just the server crate.

## 3. Deep-coupling cleanups (must-keep crates — surgical, not deletes)

These edits land in must-keep crates (allowed; only `gpui*` is off-limits):

- **telemetry**: `telemetry::event!` + `client::telemetry` used by editor,
  workspace, fs, git_ui, extension_host, project_panel, command_palette, client,
  onboarding (+~30, many themselves being deleted). After AI/collab removal,
  remove remaining `event!` call sites in survivors and drop the dep. Verify each
  survivor builds.
- **agent_settings**: relocate non-AI layout (`sidebar_side`, `WindowLayout`,
  `PanelLayout`) into `workspace` (or `settings`); strip AI-gated features:
  - git_ui: AI commit message (git_panel.rs ~2738/2762), merge-conflict AI
    indicator (conflict_view.rs ~297/554/602, project_diff.rs ~1722).
  - workspace: welcome.rs:416 AI onboarding section; multi_workspace.rs:329/402
    `enabled` gates; zed/quick_action_bar.rs agent button.
  - Then delete `agent_settings`.
- **DisableAiSettings** (workspace/multi_workspace.rs:402): keep or simplify;
  decide during agent_settings step.

## 4. Expanded removal set (leaf → root order)

Exact membership verified at each step before deletion. Build after every crate.

**Stage A — pure leaves (no dependents):**
1. editor_benchmarks
2. fs_benchmarks
3. eval_cli (drop its `agent` edge), then eval_utils
4. feedback (zed-only; drop `feedback::init` main.rs:771)
5. extensions_ui (zed-only; drop `extensions_ui::init` main.rs:778)
6. vim (drop `vim::init` main.rs:756, `vim::ModeIndicator` zed.rs:566; agent_ui dep dies with agent_ui)

**Stage B — collaboration stack:**
7. collab (server) → call, channel, notifications*, collab_ui, livekit_client/livekit_api*
   - main.rs: `collab_ui::init` (768), `call::init` (766), `channel::init` (745),
     `notifications::init` (767), `ChannelView` usage (24, 1418), `join_channel`.
   - *verify `notifications` isn't reused by non-collab toasts before deleting.

**Stage C — in-editor AI / edit-prediction stack:**
8. agent_skills, skill_creator, prompt_store
9. agent_ui (AgentPanel, InlineAssistant) → then agent
10. acp_tools, agent_servers, and `agent_client_protocol` dep (main.rs:18)
11. edit_prediction, edit_prediction_ui, edit_prediction_button*, edit_prediction_cli, zeta*/zeta2*/zeta_cli* (*verify names)
12. copilot_ui, copilot_chat, copilot
13. language_models, language_models_cloud, language_model, web_search, web_search_providers
14. providers: anthropic, open_ai, google_ai, ollama
15. agent_settings (per §3, after all AI consumers gone)

**Stage D — telemetry (last):**
16. telemetry + remaining `client::telemetry` surface in survivors.

After all: delete leftover crate dirs, remove from root `Cargo.toml`
`[workspace.members]`, `cargo build --release`.

Each removal commit: `remove: <crate> - <reason>` (Conventional Commits).

## 5. Asset slimming (Phase 1.2 / 1.3)

- Themes: delete `assets/themes/gruvbox/`, `assets/themes/one/`; in
  `assets/themes/ayu/ayu.json` keep only "Ayu Dark". Sweep `crates/theme/src/`
  for `gruvbox`/`One Dark`/`One Light` references.
- Keymaps: keep VSCode only; remove JetBrains/SublimeText/Atom/Emacs/Cursor
  keymap assets + their `BaseKeymap` variants (crates/settings/src/base_keymap_setting.rs)
  and the picker in `crates/onboarding/src/base_keymap_picker.rs`.
- `assets/settings/default.json`: `"theme": "Ayu Dark"`, `"base_keymap": "VSCode"`.

## 6. Phase 2 — wry browser side panel

- Root `Cargo.toml`: add `lb-wry` (latest).
- New `crates/browser_panel/` (`[lib] path = "src/browser_panel.rs"`):
  GPUI dock side panel hosting a `wry` WebView (vertical split from editor),
  minimal URL bar + reload. Toggle `Cmd+Shift+B`, default `http://localhost:3000`.
  WebView is a native child surface — keep fully separated from GPUI
  popups/tooltips (no overlap). Side-panel only; no tab mode.
- Register in `crates/zed/src/main.rs`.

## 7. Verification

`cargo build --release` && `cargo test --workspace` && `./script/clippy`.
Manual: file edit, TS/Rust LSP, terminal (claude/codex), Git/GitHub, Ayu Dark,
VSCode keymap, browser panel (toggle, localhost render, no overlap).

## 8. Phase 4 — public release

README (EN: intro, upstream diff, build, browser-panel usage), LICENSE-GPL +
LICENSE-APACHE with upstream attribution.
