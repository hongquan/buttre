// SPDX-License-Identifier: GPL-3.0-only
// Logging infrastructure for buttre TSF

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging for debug builds
///
/// Uses tracing framework without file I/O overhead
#[cfg(debug_assertions)]
pub fn init_logging() {
    INIT.call_once(|| {
        use tracing_subscriber::fmt;

        // Setup tracing subscriber (outputs to stderr/debugger)
        // No file I/O - much faster!
        fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_target(false)
            .with_thread_ids(false) // Disabled for performance
            .with_file(false) // Disabled for performance
            .with_line_number(false) // Disabled for performance
            .compact() // Compact format for speed
            .try_init()
            .ok();
    });
}

/// Initialize logging for release builds — WARN level only, no file output.
/// Debug-level messages (which include key characters) are suppressed to
/// prevent keystroke data from being written to disk.
#[cfg(not(debug_assertions))]
pub fn init_logging() {
    INIT.call_once(|| {
        use tracing_subscriber::fmt;
        fmt()
            .with_max_level(tracing::Level::WARN)
            .with_target(false)
            .with_ansi(false)
            .compact()
            .try_init()
            .ok();
    });
}

#[cfg(debug_assertions)]
#[inline(always)]
pub fn log_debug(msg: &str) {
    tracing::trace!("{}", msg);
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn log_debug(msg: &str) {
    // Suppressed in release â€” debug-level events are not emitted.
    let _ = msg;
}
