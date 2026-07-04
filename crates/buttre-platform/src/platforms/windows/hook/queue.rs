//! Lock-Free Queue Architecture for Hook Optimization
//!
//! ## Problem
//! The current architecture processes all input synchronously in the hook callback:
//! - Hook callback (~200-500μs total)
//!   - Mutex lock (~50-100μs)
//!   - kb.process() (~100-300μs)
//!   - SendInput (~50-100μs)
//!
//! This can cause:
//! - Dropped keystrokes if callback takes too long
//! - System freeze if hook blocks
//! - Mutex contention under fast typing
//!
//! ## Solution
//! Decouple hook callback from processing:
//! - Hook enqueues keystroke (~10-50μs) → Return immediately
//! - Background thread dequeues and processes → Async
//! - Total latency still < 2ms (imperceptible)
//!
//! ## Architecture
//! ```text
//! OS Keystroke → Hook Callback (FAST)
//!     |
//!     ├─ Create KeyEvent
//!     ├─ Push to lock-free queue
//!     └─ Return (< 50μs)
//!
//! Background Thread (ASYNC)
//!     |
//!     ├─ Pop from queue
//!     ├─ kb.process(ch)
//!     ├─ send_backspaces/send_string
//!     └─ Loop
//! ```

use buttre_core::{Action, Keyboard};
use crossbeam::queue::SegQueue;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use tracing::{debug, warn};

use super::profiling::HOOK_PROFILER;
use crate::platforms::windows::common::{send_backspaces, send_replacement, send_string};

/// Event types that can be queued
#[derive(Debug, Clone)]
pub enum KeyEvent {
    /// Character input to process
    Character { ch: char, timestamp_us: u64 },

    /// Backspace key
    Backspace { timestamp_us: u64 },

    /// Reset engine state (on mouse click, movement keys, etc.)
    Reset { timestamp_us: u64 },

    /// Shutdown signal
    Shutdown,
}

/// Statistics for queue performance
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct QueueStats {
    pub enqueued: usize,
    pub processed: usize,
    pub dropped: usize,
    pub max_queue_len: usize,
}

/// Lock-free queue processor for keyboard events
pub struct QueueProcessor {
    /// Lock-free queue (MPSC - Multiple Producer Single Consumer)
    queue: Arc<SegQueue<KeyEvent>>,

    /// Keyboard instance for processing (Phase 4, Task 3: Using RwLock)
    keyboard: Arc<RwLock<Option<Keyboard>>>,

    /// Running flag
    running: Arc<AtomicBool>,

    /// Processor thread handle
    thread: Option<JoinHandle<()>>,

    /// Queue capacity limit (to prevent memory exhaustion)
    max_queue_size: usize,
}

impl QueueProcessor {
    /// Create new queue processor
    ///
    /// # Arguments
    /// * `keyboard` - Shared keyboard instance (RwLock for lower contention)
    /// * `max_queue_size` - Maximum queue size (default: 1000)
    pub fn new(keyboard: Arc<RwLock<Option<Keyboard>>>, max_queue_size: usize) -> Self {
        Self {
            queue: Arc::new(SegQueue::new()),
            keyboard,
            running: Arc::new(AtomicBool::new(false)),
            thread: None,
            max_queue_size,
        }
    }

    /// Start the background processor thread
    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.running.load(Ordering::Acquire) {
            anyhow::bail!("Queue processor already running");
        }

        self.running.store(true, Ordering::Release);

        let queue = self.queue.clone();
        let keyboard = self.keyboard.clone();
        let running = self.running.clone();

        let thread = thread::Builder::new()
            .name("buttre-queue-processor".to_string())
            .spawn(move || {
                Self::processor_loop(queue, keyboard, running);
            })?;

        self.thread = Some(thread);

        tracing::info!("Queue processor thread started");
        Ok(())
    }

    /// Stop the processor thread
    pub fn stop(&mut self) {
        if !self.running.load(Ordering::Acquire) {
            return;
        }

        // Signal shutdown
        self.running.store(false, Ordering::Release);
        self.queue.push(KeyEvent::Shutdown);

        // Wait for thread to finish
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }

        tracing::info!("Queue processor thread stopped");
    }

    /// Enqueue a keystroke event
    ///
    /// Returns true if enqueued, false if queue is full
    #[inline]
    pub fn enqueue(&self, event: KeyEvent) -> bool {
        // Check queue size to prevent memory exhaustion
        if self.queue.len() >= self.max_queue_size {
            warn!(
                "Queue full ({} events), dropping keystroke",
                self.max_queue_size
            );
            return false;
        }

        self.queue.push(event);
        true
    }

    /// Get current queue length
    #[inline]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if queue is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Main processor loop (runs in background thread)
    fn processor_loop(
        queue: Arc<SegQueue<KeyEvent>>,
        keyboard: Arc<RwLock<Option<Keyboard>>>,
        running: Arc<AtomicBool>,
    ) {
        let mut processed_count = 0;
        let mut last_log_time = std::time::Instant::now();

        while running.load(Ordering::Acquire) {
            // Try to pop event from queue
            match queue.pop() {
                Some(KeyEvent::Shutdown) => {
                    debug!("Received shutdown signal, exiting processor loop");
                    break;
                }

                Some(KeyEvent::Character { ch, timestamp_us }) => {
                    Self::process_character(&keyboard, ch, timestamp_us);
                    processed_count += 1;
                }

                Some(KeyEvent::Backspace { timestamp_us }) => {
                    Self::process_backspace(&keyboard, timestamp_us);
                    processed_count += 1;
                }

                Some(KeyEvent::Reset { .. }) => {
                    Self::reset_keyboard(&keyboard);
                    processed_count += 1;
                }

                None => {
                    // Queue empty, sleep briefly to avoid busy-wait
                    thread::sleep(Duration::from_micros(100));
                }
            }

            // Log stats every 10 seconds
            if last_log_time.elapsed().as_secs() >= 10 && processed_count > 0 {
                debug!(
                    "Queue processor: {} events processed, queue len: {}",
                    processed_count,
                    queue.len()
                );
                processed_count = 0;
                last_log_time = std::time::Instant::now();
            }
        }

        debug!("Queue processor loop exited");
    }

    /// Process a character input
    fn process_character(keyboard: &Arc<RwLock<Option<Keyboard>>>, ch: char, _timestamp_us: u64) {
        // Try to get write lock (non-blocking)
        let result = if let Ok(mut kb_opt) = keyboard.try_write() {
            if let Some(ref mut kb) = *kb_opt {
                match kb.process(ch) {
                    Ok(actions) => {
                        // Process all actions
                        let mut main_action = Action::DoNothing;

                        for action in actions {
                            match action {
                                Action::ShowCandidates { candidates, input } => {
                                    // Handle candidate display
                                    use crate::platforms::windows::common::{
                                        get_candidates_text_len, show_candidates,
                                    };
                                    let old_candidates_len = get_candidates_text_len();

                                    if let Some(candidates_text) =
                                        show_candidates(candidates, input)
                                    {
                                        main_action = match main_action {
                                            Action::Replace {
                                                backspace_count,
                                                text,
                                            } => Action::Replace {
                                                backspace_count: backspace_count
                                                    + old_candidates_len,
                                                text: text + &candidates_text,
                                            },
                                            Action::Commit(text) => Action::Replace {
                                                backspace_count: old_candidates_len,
                                                text: text + &candidates_text,
                                            },
                                            Action::DoNothing => Action::Replace {
                                                backspace_count: old_candidates_len,
                                                text: candidates_text,
                                            },
                                            other => other,
                                        };
                                    }
                                }
                                Action::HideCandidates => {
                                    use crate::platforms::windows::common::{
                                        get_candidates_text_len, hide_candidates,
                                    };
                                    let candidates_len = get_candidates_text_len();
                                    if candidates_len > 0 {
                                        send_backspaces(candidates_len);
                                    }
                                    hide_candidates();
                                }
                                other => {
                                    if matches!(main_action, Action::DoNothing) {
                                        main_action = other;
                                    }
                                }
                            }
                        }

                        main_action
                    }
                    Err(e) => {
                        warn!("Keyboard process error: {}", e);
                        Action::DoNothing
                    }
                }
            } else {
                Action::DoNothing
            }
        } else {
            // Lock busy
            HOOK_PROFILER.record_lock_busy();
            debug!("Keyboard lock busy in queue processor");
            Action::DoNothing
        };

        // Execute action
        match result {
            Action::Replace {
                backspace_count,
                text,
            } => {
                if backspace_count > 0 || !text.is_empty() {
                    send_replacement(backspace_count, &text);
                }
            }
            Action::Commit(text) => {
                if !text.is_empty() {
                    send_string(&text);
                }
            }
            _ => {}
        }
    }

    /// Process backspace
    fn process_backspace(keyboard: &Arc<RwLock<Option<Keyboard>>>, _timestamp_us: u64) {
        use crate::platforms::windows::common::{hide_candidates, is_candidates_showing};

        // Hide candidates if showing
        if is_candidates_showing() {
            hide_candidates();
        }

        let result = if let Ok(mut kb_opt) = keyboard.try_write() {
            if let Some(ref mut kb) = *kb_opt {
                match kb.backspace() {
                    Ok(action) => action,
                    Err(e) => {
                        warn!("Keyboard backspace error: {}", e);
                        Action::DoNothing
                    }
                }
            } else {
                Action::DoNothing
            }
        } else {
            HOOK_PROFILER.record_lock_busy();
            Action::DoNothing
        };

        if let Action::Replace {
            backspace_count,
            text,
        } = result
        {
            if backspace_count > 0 || !text.is_empty() {
                send_replacement(backspace_count, &text);
            }
        }
    }

    /// Reset keyboard state
    fn reset_keyboard(keyboard: &Arc<RwLock<Option<Keyboard>>>) {
        if let Ok(mut kb_opt) = keyboard.try_write() {
            if let Some(ref mut kb) = *kb_opt {
                kb.reset();
            }
        }
    }
}

impl Drop for QueueProcessor {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Get current timestamp in microseconds
#[inline]
pub fn timestamp_us() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as u64)
        .unwrap_or(0)
}
