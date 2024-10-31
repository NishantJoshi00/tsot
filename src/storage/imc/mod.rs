#[cfg(feature = "async")]
mod async_impl;
#[cfg(feature = "sync")]
mod sync_impl;

use core::sync::atomic::AtomicI64;
use dashmap::DashMap;
use std::sync::Arc;

type ArcDashMap<K, V> = Arc<DashMap<K, V>>;

/// In-memory cache module implementation.
///
/// Provides a thread-safe storage mechanism for string values with optional
/// expiration times. Uses [`DashMap`] as the underlying concurrent hash map
/// wrapped in an [`Arc`] for safe sharing across threads.
///
/// The stored values are tuples of (String, Option<u64>) where:
/// - The String is the stored value
/// - The Option<u64> is the optional expiration time in Unix timestamp seconds
#[derive(Clone)]
pub struct IMCModule {
    /// Thread-safe storage for string values and their expiration times
    string_store: Arc<DashMap<String, (String, Option<u64>)>>,
    /// Thread-safe storage for binary data values and their expiration times
    data_store: ArcDashMap<String, (Vec<u8>, Option<u64>)>,

    atomic_store: Arc<DashMap<String, AtomicI64>>,
}

/// Configuration struct for IMCModule.
///
/// Currently empty but provides extensibility for future configuration options
/// such as default expiration times, maximum cache size, etc.
pub struct IMCConfig {}

impl IMCModule {
    /// Creates a new instance of IMCModule.
    ///
    /// # Arguments
    /// * `_config` - Configuration options for the cache (currently unused)
    ///
    /// # Returns
    /// * `Self` - A new instance of IMCModule with an empty cache
    pub fn new(_config: IMCConfig) -> Self {
        Self {
            string_store: Arc::new(DashMap::new()),
            data_store: Arc::new(DashMap::new()),
            atomic_store: Arc::new(DashMap::new()),
        }
    }
}

/// Gets the current Unix timestamp in seconds.
///
/// Helper function that returns the current time as seconds since the Unix epoch.
/// Used for calculating and checking expiration times.
///
/// # Returns
/// * `u64` - Current Unix timestamp in seconds
///
/// # Panics
/// Panics if the system time is set to before the Unix epoch (1970-01-01 00:00:00 UTC).
/// This could happen if:
/// - The system clock is incorrectly set
/// - The function is running in a time machine
/// - The system is experiencing severe clock skew
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
