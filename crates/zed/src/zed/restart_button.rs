use gpui::{App, TaskExt as _, WindowHandle};
use ui::{ButtonCommon, Clickable, Context, IconButton, IconName, IconSize, Render, Tooltip, Window};
use util::ResultExt as _;
use workspace::{CloseIntent, HideStatusItem, ItemHandle, MultiWorkspace, StatusItemView};

/// Status bar button that relaunches kid in place, picking up a freshly built
/// binary without a manual force-quit. Mirrors the app quit flow so dirty
/// buffers are still saved/discarded before exiting.
pub struct RestartButton;

impl RestartButton {
    pub fn new() -> Self {
        Self
    }
}

impl Render for RestartButton {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl ui::IntoElement {
        IconButton::new("restart-kid", IconName::RotateCcw)
            .icon_size(IconSize::Small)
            .tooltip(Tooltip::text("kid 재시작"))
            .on_click(|_, _window, cx| restart_kid(cx))
    }
}

impl StatusItemView for RestartButton {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        None
    }
}

/// Saves or discards dirty items across every window (prompting only when there
/// are unsaved changes), flushes workspace serialization, then restarts the app.
fn restart_kid(cx: &mut App) {
    cx.spawn(async move |cx| {
        let mut workspace_windows: Vec<WindowHandle<MultiWorkspace>> = cx.update(|cx| {
            cx.windows()
                .into_iter()
                .filter_map(|window| window.downcast::<MultiWorkspace>())
                .collect::<Vec<_>>()
        });

        // Prompt in the active window first before switching to other windows.
        cx.update(|cx| {
            workspace_windows.sort_by_key(|window| window.is_active(cx) == Some(false));
        });

        // If the user cancels any save prompt, keep the app running.
        for window in &workspace_windows {
            let window = *window;
            let workspaces = window
                .update(cx, |multi_workspace, _, _cx| {
                    multi_workspace.workspaces().cloned().collect::<Vec<_>>()
                })
                .log_err();

            let Some(workspaces) = workspaces else {
                continue;
            };

            for workspace in workspaces {
                if let Some(should_close) = window
                    .update(cx, |multi_workspace, window, cx| {
                        multi_workspace.activate(workspace.clone(), None, window, cx);
                        window.activate_window();
                        workspace.update(cx, |workspace, cx| {
                            workspace.prepare_to_close(CloseIntent::Quit, window, cx)
                        })
                    })
                    .log_err()
                {
                    if !should_close.await? {
                        return anyhow::Ok(());
                    }
                }
            }
        }

        // Flush all pending workspace serialization so session/window ids are
        // up-to-date in the database before the process exits.
        let mut flush_tasks = Vec::new();
        for window in &workspace_windows {
            window
                .update(cx, |multi_workspace, window, cx| {
                    for workspace in multi_workspace.workspaces() {
                        flush_tasks.push(workspace.update(cx, |workspace, cx| {
                            workspace.flush_serialization(window, cx)
                        }));
                    }
                    flush_tasks.append(&mut multi_workspace.take_pending_removal_tasks());
                    flush_tasks.push(multi_workspace.flush_serialization());
                })
                .log_err();
        }
        futures::future::join_all(flush_tasks).await;

        cx.update(|cx| cx.restart());
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
