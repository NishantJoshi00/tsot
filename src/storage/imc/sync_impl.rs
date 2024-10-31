//! In-memory cache implementation for string storage with optional expiration.
//!
//! This module provides a thread-safe, in-memory cache implementation using DashMap
//! as the underlying concurrent hash map. It supports storing string values with
//! optional expiration times and implements the [`StringStorage`] and
//! [`StringStorageWithExpiry`] traits.

use core::sync::atomic::AtomicI64;

use crate::sync::{
    AtomicStorage, RawStorage, RawStorageWithExpiry, StringStorage, StringStorageWithExpiry,
};

use super::{now, IMCModule};

impl StringStorageWithExpiry for IMCModule {
    /// Stores a string value with an optional expiration time.
    ///
    /// Calculates the absolute expiration time by adding the provided duration
    /// to the current time.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The string value to store
    /// * `expiry` - Optional expiration duration in seconds from now
    ///
    /// # Returns
    /// * `Ok(StoreState::New)` - If the key did not exist
    /// * `Ok(StoreState::Updated)` - If the key existed and was updated
    fn store_with_expiry(
        &self,
        key: String,
        value: String,
        expiry: Option<u64>,
    ) -> Result<crate::types::StoreState, crate::errors::StorageError> {
        let current_time = expiry.map(|e| now() + e);
        let output = self.string_store.insert(key, (value, current_time));

        match output {
            None => Ok(crate::types::StoreState::New),
            Some(_) => Ok(crate::types::StoreState::Updated),
        }
    }
}

impl StringStorage for IMCModule {
    /// Loads a string value if it exists and hasn't expired.
    ///
    /// If the value has expired, it is automatically deleted and None is returned.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be loaded
    ///
    /// # Returns
    /// * `Ok(Some(String))` - If the key exists and hasn't expired
    /// * `Ok(None)` - If the key doesn't exist or has expired
    ///
    fn load_string(&self, key: String) -> Result<Option<String>, crate::errors::StorageError> {
        match self.string_store.get(&key) {
            Some(value) => {
                let (inner_value, expiry) = value.value();
                match expiry {
                    Some(expiry) if expiry < &now() => {
                        self.delete_string(key.clone())?;
                        Ok(None)
                    }
                    _ => Ok(Some(inner_value.clone())),
                }
            }
            None => Ok(None),
        }
    }

    /// Deletes a string value.
    ///
    /// Removes the value associated with the given key. If the key doesn't exist,
    /// the operation is still considered successful.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be deleted
    ///
    /// # Returns
    /// * `Ok(())` - The operation was successful (whether or not the key existed)
    fn delete_string(&self, key: String) -> Result<(), crate::errors::StorageError> {
        self.string_store.remove(&key);
        Ok(())
    }
}

impl RawStorageWithExpiry for IMCModule {
    fn store_raw_with_expiry(
        &self,
        key: String,
        value: Vec<u8>,
        expiry: Option<u64>,
    ) -> Result<crate::types::StoreState, crate::errors::StorageError> {
        let current_time = expiry.map(|e| now() + e);
        let output = self.data_store.insert(key, (value, current_time));

        match output {
            None => Ok(crate::types::StoreState::New),
            Some(_) => Ok(crate::types::StoreState::Updated),
        }
    }
}

impl RawStorage for IMCModule {
    fn load_raw(&self, key: String) -> Result<Option<Vec<u8>>, crate::errors::StorageError> {
        match self.data_store.get(&key) {
            Some(value) => {
                let (inner_value, expiry) = value.value();
                match expiry {
                    Some(expiry) if expiry < &now() => {
                        self.delete_raw(key.clone())?;
                        Ok(None)
                    }
                    _ => Ok(Some(inner_value.clone())),
                }
            }
            None => Ok(None),
        }
    }

    fn delete_raw(&self, key: String) -> Result<(), crate::errors::StorageError> {
        self.data_store.remove(&key);
        Ok(())
    }
}

impl AtomicStorage for IMCModule {
    fn atomic_store(
        &self,
        key: String,
        value: i64,
    ) -> Result<crate::types::StoreState, crate::errors::StorageError> {
        let output = self.atomic_store.insert(key, AtomicI64::new(value));

        match output {
            None => Ok(crate::types::StoreState::New),
            Some(_) => Ok(crate::types::StoreState::Updated),
        }
    }

    fn atomic_load(&self, key: String) -> Result<Option<i64>, crate::errors::StorageError> {
        match self.atomic_store.get(&key) {
            Some(value) => Ok(Some(value.load(std::sync::atomic::Ordering::SeqCst))),
            None => Ok(None),
        }
    }

    fn atomic_delete(&self, key: String) -> Result<(), crate::errors::StorageError> {
        self.atomic_store.remove(&key);
        Ok(())
    }

    fn atomic_increment(
        &self,
        key: String,
        value: i64,
    ) -> Result<Option<i64>, crate::errors::StorageError> {
        let output = self.atomic_store.get(&key);

        match output {
            Some(atomic) => Ok(Some(
                atomic.fetch_add(value, std::sync::atomic::Ordering::SeqCst),
            )),
            None => Ok(None),
        }
    }
}
