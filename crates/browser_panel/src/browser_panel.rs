//! A browser side panel backed by a native [`wry`] WebView.
//!
//! The native WebView is created as a child of the GPUI window's content view via
//! [`wry::WebViewBuilder::build_as_child`]. Because the native view renders itself outside of
//! GPUI's painting pipeline, [`WebViewElement`] is responsible for keeping the native view's
//! bounds in sync with the GPUI layout: during `prepaint` it pushes the element's computed bounds
//! down to the WebView and toggles visibility when the element scrolls out of the viewport.

use std::cell::Cell;
use std::rc::Rc;

use anyhow::{Context as _, Result};
use editor::Editor;
use gpui::{
    Action, App, Bounds, Context, Element, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    GlobalElementId, InspectorElementId, IntoElement, LayoutId, Pixels, Render, Style, Window,
    actions, point, px,
};
use ui::prelude::*;
use ui::Tooltip;
use util::ResultExt as _;
use workspace::Workspace;
use workspace::dock::{DockPosition, Panel, PanelEvent};

const DEFAULT_URL: &str = "http://localhost:3000";

actions!(
    browser_panel,
    [
        /// Toggles focus on the browser panel.
        ToggleFocus
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _: &mut Context<Workspace>| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<BrowserPanel>(window, cx);
        });
    })
    .detach();
}

/// Shared handle to the native WebView. Held by both the panel and the element so the element can
/// reposition the native view during layout.
type SharedWebView = Rc<wry::WebView>;

pub struct BrowserPanel {
    focus_handle: FocusHandle,
    url_editor: Entity<Editor>,
    /// The native WebView. Created lazily on first render, because a [`Window`] handle (which
    /// implements [`raw_window_handle::HasWindowHandle`]) is required to attach the native view as
    /// a child of the window's content view.
    webview: Option<SharedWebView>,
    /// The URL currently loaded (or requested to load) in the WebView.
    current_url: String,
}

impl BrowserPanel {
    pub async fn load(
        workspace: gpui::WeakEntity<Workspace>,
        mut cx: gpui::AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |_workspace, window, cx| {
            cx.new(|cx| BrowserPanel::new(window, cx))
        })
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Enter a URL", window, cx);
            editor.set_text(DEFAULT_URL, window, cx);
            editor
        });

        Self {
            focus_handle: cx.focus_handle(),
            url_editor,
            webview: None,
            current_url: DEFAULT_URL.to_string(),
        }
    }

    /// Ensure the native WebView exists. This must run while a [`Window`] is available so we can
    /// obtain a window handle to parent the native view.
    fn ensure_webview(&mut self, window: &mut Window) -> Option<SharedWebView> {
        if let Some(webview) = &self.webview {
            return Some(webview.clone());
        }

        let webview = wry::WebViewBuilder::new()
            .with_url(&self.current_url)
            .with_bounds(wry::Rect {
                position: wry::dpi::LogicalPosition::new(0.0, 0.0).into(),
                size: wry::dpi::LogicalSize::new(640.0, 480.0).into(),
            })
            .build_as_child(window)
            .context("failed to create browser WebView")
            .log_err()?;

        let webview = Rc::new(webview);
        self.webview = Some(webview.clone());
        Some(webview)
    }

    fn load_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let url = self.url_editor.read(cx).text(cx);
        let url = url.trim().to_string();
        if url.is_empty() {
            return;
        }
        self.current_url = url.clone();
        if let Some(webview) = self.ensure_webview(window) {
            webview.load_url(&url).log_err();
        }
        cx.notify();
    }

    fn reload(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(webview) = self.ensure_webview(window) {
            webview.reload().log_err();
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.load_url(window, cx);
    }
}

impl Focusable for BrowserPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for BrowserPanel {}

impl Panel for BrowserPanel {
    fn persistent_name() -> &'static str {
        "BrowserPanel"
    }

    fn panel_key() -> &'static str {
        "BrowserPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(480.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Public)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Browser Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn set_active(&mut self, active: bool, window: &mut Window, _cx: &mut Context<Self>) {
        // Showing/hiding is also handled by the element's viewport check, but updating here avoids
        // a one-frame flash of a stale native view when the panel is toggled.
        if let Some(webview) = self.webview.clone() {
            webview.set_visible(active).log_err();
        } else if active {
            self.ensure_webview(window);
        }
    }

    fn activation_priority(&self) -> u32 {
        5
    }
}

impl Render for BrowserPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let webview = self.ensure_webview(window);

        v_flex()
            .key_context("BrowserPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm))
            .size_full()
            .child(
                h_flex()
                    .flex_none()
                    .w_full()
                    .gap_1()
                    .p_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(div().flex_1().child(self.url_editor.clone()))
                    .child(
                        IconButton::new("browser-reload", IconName::RotateCw)
                            .tooltip(Tooltip::text("Reload"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.reload(window, cx);
                            })),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .when_some(webview, |this, webview| {
                        this.child(WebViewElement::new(webview))
                    }),
            )
    }
}

/// A GPUI [`Element`] that positions a native [`wry::WebView`] to match its computed bounds.
///
/// `paint` is intentionally empty — the native NSView renders itself. All the work happens in
/// `prepaint`, where we translate GPUI logical pixels into wry's logical units and either show or
/// hide the native view depending on whether the element is within the window's viewport.
struct WebViewElement {
    webview: SharedWebView,
    /// Tracks the last visibility we pushed to the native view so we avoid redundant calls.
    last_visible: Rc<Cell<bool>>,
}

impl WebViewElement {
    fn new(webview: SharedWebView) -> Self {
        Self {
            webview,
            last_visible: Rc::new(Cell::new(true)),
        }
    }
}

impl IntoElement for WebViewElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for WebViewElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::Name("browser-webview".into()))
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.0;
        style.size.width = gpui::relative(1.).into();
        style.size.height = gpui::relative(1.).into();
        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        // Determine whether the element is within the visible viewport. When it scrolls out of
        // view we hide the native view, since it would otherwise float over unrelated content.
        let viewport = window.viewport_size();
        let viewport_bounds = Bounds {
            origin: point(px(0.), px(0.)),
            size: viewport,
        };
        let visible = bounds.intersects(&viewport_bounds) && bounds.size.width > px(0.);

        if visible {
            self.webview
                .set_bounds(wry::Rect {
                    position: wry::dpi::LogicalPosition::new(
                        bounds.origin.x.to_f64(),
                        bounds.origin.y.to_f64(),
                    )
                    .into(),
                    size: wry::dpi::LogicalSize::new(
                        bounds.size.width.to_f64(),
                        bounds.size.height.to_f64(),
                    )
                    .into(),
                })
                .log_err();
        }

        if self.last_visible.get() != visible {
            self.webview.set_visible(visible).log_err();
            self.last_visible.set(visible);
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        // The native NSView paints itself; nothing to do here.
    }
}
