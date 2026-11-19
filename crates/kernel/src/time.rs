//! Guest clocks.

use std::sync::LazyLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static BOOT: LazyLock<Instant> = LazyLock::new(Instant::now);

/// CLOCK_MONOTONIC / CLOCK_BOOTTIME: time since the guest "booted". Starts at
/// a few seconds so code that treats 0 as "unset" behaves.
pub fn monotonic() -> Duration {
    BOOT.elapsed() + Duration::from_secs(5)
}

pub fn realtime() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
}

/// Host Instant corresponding to a guest absolute time on `clock`.
pub fn deadline(clock: u32, abs: Duration) -> Instant {
    let now = if clock == 0 { realtime() } else { monotonic() };
    Instant::now() + abs.saturating_sub(now)
}

pub fn init() {
    LazyLock::force(&BOOT);
}
