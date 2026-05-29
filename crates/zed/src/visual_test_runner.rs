//! Visual Test Runner (stub)
//!
//! This binary previously ran visual regression tests for Zed's UI by capturing
//! screenshots of real windows and comparing them against baseline images.
//!
//! The visual test suite exercised features (the AI agent panel, agent threads,
//! the multi-workspace sidebar, prompt store, language models, and several
//! settings sub-pages) that have been removed from this fork. With those crates
//! gone the runner has no meaningful UI to capture, so it has been reduced to a
//! stub that compiles under the `visual-tests` feature and exits without doing
//! any work.
//!
//! The `[[bin]]` target and the `visual-tests` feature are kept so existing CI
//! invocations (`cargo build -p zed --bin zed_visual_test_runner --features
//! visual-tests`) continue to succeed.

fn main() {
    eprintln!(
        "zed_visual_test_runner: the visual regression suite has been removed in this fork; \
         nothing to run."
    );
}
