//! Hook Callback Profiling
//!
//! Provides low-overhead timing measurements for hook callbacks.
//! Uses atomic counters to avoid locks and minimize performance impact.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

/// Global profiling statistics
pub struct HookProfiler {
    /// Total number of hook callbacks
    pub total_calls: AtomicUsize,
    
    /// Total time spent in hook callback (nanoseconds)
    pub total_time_ns: AtomicU64,
    
    /// Maximum time spent in single callback (nanoseconds)
    pub max_time_ns: AtomicU64,
    
    /// Minimum time spent in single callback (nanoseconds)
    pub min_time_ns: AtomicU64,
    
    /// Number of callbacks that processed Vietnamese input
    pub vietnamese_calls: AtomicUsize,
    
    /// Number of callbacks that were passthrough (English/modifiers)
    pub passthrough_calls: AtomicUsize,
    
    /// Number of times lock was busy (try_lock failed)
    pub lock_busy_count: AtomicUsize,
}

impl HookProfiler {
    pub const fn new() -> Self {
        Self {
            total_calls: AtomicUsize::new(0),
            total_time_ns: AtomicU64::new(0),
            max_time_ns: AtomicU64::new(0),
            min_time_ns: AtomicU64::new(u64::MAX),
            vietnamese_calls: AtomicUsize::new(0),
            passthrough_calls: AtomicUsize::new(0),
            lock_busy_count: AtomicUsize::new(0),
        }
    }
    
    /// Record a hook callback timing
    #[inline]
    pub fn record_callback(&self, duration_ns: u64, was_vietnamese: bool) {
        self.total_calls.fetch_add(1, Ordering::Relaxed);
        self.total_time_ns.fetch_add(duration_ns, Ordering::Relaxed);
        
        if was_vietnamese {
            self.vietnamese_calls.fetch_add(1, Ordering::Relaxed);
        } else {
            self.passthrough_calls.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update max
        let mut current_max = self.max_time_ns.load(Ordering::Relaxed);
        while duration_ns > current_max {
            match self.max_time_ns.compare_exchange_weak(
                current_max,
                duration_ns,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }
        
        // Update min
        let mut current_min = self.min_time_ns.load(Ordering::Relaxed);
        while duration_ns < current_min {
            match self.min_time_ns.compare_exchange_weak(
                current_min,
                duration_ns,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_min = x,
            }
        }
    }
    
    /// Record a lock busy event
    #[inline]
    pub fn record_lock_busy(&self) {
        self.lock_busy_count.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get statistics snapshot
    pub fn get_stats(&self) -> ProfileStats {
        let total = self.total_calls.load(Ordering::Relaxed);
        let total_ns = self.total_time_ns.load(Ordering::Relaxed);
        let max_ns = self.max_time_ns.load(Ordering::Relaxed);
        let min_ns = self.min_time_ns.load(Ordering::Relaxed);
        let vietnamese = self.vietnamese_calls.load(Ordering::Relaxed);
        let passthrough = self.passthrough_calls.load(Ordering::Relaxed);
        let lock_busy = self.lock_busy_count.load(Ordering::Relaxed);
        
        let avg_ns = if total > 0 { total_ns / total as u64 } else { 0 };
        
        ProfileStats {
            total_calls: total,
            avg_us: (avg_ns as f64) / 1000.0,
            max_us: (max_ns as f64) / 1000.0,
            min_us: if min_ns == u64::MAX { 0.0 } else { (min_ns as f64) / 1000.0 },
            vietnamese_calls: vietnamese,
            passthrough_calls: passthrough,
            lock_busy_count: lock_busy,
        }
    }
    
    /// Reset all statistics
    pub fn reset(&self) {
        self.total_calls.store(0, Ordering::Relaxed);
        self.total_time_ns.store(0, Ordering::Relaxed);
        self.max_time_ns.store(0, Ordering::Relaxed);
        self.min_time_ns.store(u64::MAX, Ordering::Relaxed);
        self.vietnamese_calls.store(0, Ordering::Relaxed);
        self.passthrough_calls.store(0, Ordering::Relaxed);
        self.lock_busy_count.store(0, Ordering::Relaxed);
    }
}

/// Snapshot of profiling statistics
#[derive(Debug, Clone)]
pub struct ProfileStats {
    pub total_calls: usize,
    pub avg_us: f64,
    pub max_us: f64,
    pub min_us: f64,
    pub vietnamese_calls: usize,
    pub passthrough_calls: usize,
    pub lock_busy_count: usize,
}

impl ProfileStats {
    /// Check if performance is acceptable
    pub fn is_acceptable(&self) -> bool {
        // Target: avg < 500μs (0.5ms), max < 2000μs (2ms)
        self.avg_us < 500.0 && self.max_us < 2000.0
    }
    
    /// Get performance grade
    pub fn grade(&self) -> &'static str {
        if self.avg_us < 100.0 && self.max_us < 500.0 {
            "EXCELLENT"
        } else if self.avg_us < 300.0 && self.max_us < 1000.0 {
            "GOOD"
        } else if self.avg_us < 500.0 && self.max_us < 2000.0 {
            "ACCEPTABLE"
        } else {
            "NEEDS OPTIMIZATION"
        }
    }
}

impl std::fmt::Display for ProfileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Hook Callback Performance Profile")?;
        writeln!(f, "==================================")?;
        writeln!(f, "Total Callbacks: {}", self.total_calls)?;
        writeln!(f, "  Vietnamese: {} ({:.1}%)", 
            self.vietnamese_calls, 
            if self.total_calls > 0 { 
                (self.vietnamese_calls as f64 / self.total_calls as f64) * 100.0 
            } else { 0.0 }
        )?;
        writeln!(f, "  Passthrough: {} ({:.1}%)", 
            self.passthrough_calls,
            if self.total_calls > 0 { 
                (self.passthrough_calls as f64 / self.total_calls as f64) * 100.0 
            } else { 0.0 }
        )?;
        writeln!(f, "  Lock Busy: {}", self.lock_busy_count)?;
        writeln!(f)?;
        writeln!(f, "Timing (microseconds):")?;
        writeln!(f, "  Average: {:.2} μs", self.avg_us)?;
        writeln!(f, "  Minimum: {:.2} μs", self.min_us)?;
        writeln!(f, "  Maximum: {:.2} μs", self.max_us)?;
        writeln!(f)?;
        writeln!(f, "Performance Grade: {}", self.grade())?;
        writeln!(f, "Target: avg < 500μs, max < 2000μs")?;
        Ok(())
    }
}

/// Global profiler instance
pub static HOOK_PROFILER: HookProfiler = HookProfiler::new();

/// Timer guard for automatic timing measurement
pub struct ProfileTimer {
    start: Instant,
    was_vietnamese: bool,
}

impl ProfileTimer {
    /// Start timing a hook callback
    #[inline]
    pub fn start(was_vietnamese: bool) -> Self {
        Self {
            start: Instant::now(),
            was_vietnamese,
        }
    }
    
    /// Mark this callback as processing Vietnamese input
    #[inline]
    pub fn mark_vietnamese(&mut self) {
        self.was_vietnamese = true;
    }
}

impl Drop for ProfileTimer {
    #[inline]
    fn drop(&mut self) {
        let duration_ns = self.start.elapsed().as_nanos() as u64;
        HOOK_PROFILER.record_callback(duration_ns, self.was_vietnamese);
    }
}
