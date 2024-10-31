//! In-memory cache implementation for string storage with optional expiration.
//!
//! This module provides a thread-safe, in-memory cache implementation using DashMap
//! as the underlying concurrent hash map. It supports storing string values with
//! optional expiration times and implements the [`StringStorage`] and
//! [`StringStorageWithExpiry`] traits.

use core::sync::atomic::AtomicI64;

use super::{now, IMCModule};
use crate::asynchronous::{
    AtomicStorage, RawStorage, RawStorageWithExpiry, StringStorage, StringStorageWithExpiry,
};
use async_trait::async_trait;

#[async_trait]
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
    async fn store_with_expiry(
        &self,
        key: String,
        value: String,
        expiry: Option<u64>,
    ) -> Result<crate::types::StoreState, crate::errors::StorageError> {
        // Using tokio::task::spawn_blocking for potentially lengthy operations
        let self = self.clone();
        tokio::task::spawn_blocking(move || {
            let current_time = expiry.map(|e| now() + e);
            let output = self.string_store.insert(key, (value, current_time));

            match output {
                None => Ok(crate::types::StoreState::New),
                Some(_) => Ok(crate::types::StoreState::Updated),
            }
        })
        .await
        .unwrap_or_else(|e| Err(crate::errors::StorageError::JoinError(e)))
    }
}

#[async_trait]
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
    async fn load_string(
        &self,
        key: String,
    ) -> Result<Option<String>, crate::errors::StorageError> {
        let self = self.clone();
        // Using tokio::task::spawn_blocking since DashMap operations might be CPU-intensive
        tokio::task::spawn_blocking(move || {
            match self.string_store.get(&key) {
                Some(value) => {
                    let (inner_value, expiry) = value.value();
                    match expiry {
                        Some(expiry) if expiry < &now() => {
                            // Note: This is now potentially problematic as it's a recursive async call
                            // We should handle this differently in a real implementation
                            self.string_store.remove(&key);
                            Ok(None)
                        }
                        _ => Ok(Some(inner_value.clone())),
                    }
                }
                None => Ok(None), // Changed from todo!() to returning None
            }
        })
        .await
        .unwrap_or_else(|e| Err(crate::errors::StorageError::JoinError(e)))
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
    async fn delete_string(&self, key: String) -> Result<(), crate::errors::StorageError> {
        let self = self.clone();
        tokio::task::spawn_blocking(move || {
            self.string_store.remove(&key);
            Ok(())
        })
        .await
        .unwrap_or_else(|e| Err(crate::errors::StorageError::JoinError(e)))
    }
}

#[async_trait]
impl RawStorageWithExpiry for IMCModule {
    /// Stores a binary value with an optional expiration time.
    ///
    /// Calculates the absolute expiration time by adding the provided duration
    /// to the current time.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The binary value to store
    /// * `expiry` - Optional expiration duration in seconds from now
    ///
    /// # Returns
    /// * `Ok(StoreState::New)` - If the key did not exist
    /// * `Ok(StoreState::Updated)` - If the key existed and was updated
    /// * `Err(StorageError)` - If an error occurred during storage
    ///
    async fn store_raw_with_expiry(
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

#[async_trait]
impl RawStorage for IMCModule {
    /// Loads a binary value if it exists and hasn't expired.
    ///
    /// If the value has expired, it is automatically deleted and None is returned.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be loaded
    ///
    /// # Returns
    /// * `Ok(Some(Vec<u8>))` - If the key exists and hasn't expired
    async fn load_raw(&self, key: String) -> Result<Option<Vec<u8>>, crate::errors::StorageError> {
        match self.data_store.get(&key) {
            Some(value) => {
                let (inner_value, expiry) = value.value();
                match expiry {
                    Some(expiry) if expiry < &now() => {
                        self.delete_raw(key.clone()).await?;
                        Ok(None)
                    }
                    _ => Ok(Some(inner_value.clone())),
                }
            }
            None => Ok(None),
        }
    }

    /// Deletes a binary value.
    ///
    /// Removes the value associated with the given key. If the key doesn't exist,
    /// the operation is still considered successful.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be deleted
    ///
    /// # Returns
    /// * `Ok(())` - The operation was successful (whether or not the key existed)
    /// * `Err(StorageError)` - If an error occurred during deletion
    /// * `Err(JoinError)` - If an error occurred while joining the async task
    ///
    async fn delete_raw(&self, key: String) -> Result<(), crate::errors::StorageError> {
        self.data_store.remove(&key);
        Ok(())
    }
}

#[async_trait]
impl AtomicStorage for IMCModule {
    /// Stores an atomic integer value.
    ///
    /// If the key already exists, the value is updated.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The integer value to store
    /// # Returns
    /// * `Ok(StoreState::New)` - If the key did not exist
    /// * `Ok(StoreState::Updated)` - If the key existed and was updated
    async fn atomic_store(
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

    /// Loads an atomic integer value if it exists.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be loaded
    ///
    /// # Returns
    /// * `Ok(Some(i64))` - If the key exists
    /// * `Ok(None)` - If the key doesn't exist
    /// * `Err(StorageError)` - If an error occurred during loading
    async fn atomic_load(&self, key: String) -> Result<Option<i64>, crate::errors::StorageError> {
        match self.atomic_store.get(&key) {
            Some(value) => Ok(Some(value.load(std::sync::atomic::Ordering::SeqCst))),
            None => Ok(None),
        }
    }

    /// Deletes an atomic integer value.
    ///
    /// Removes the value associated with the given key. If the key doesn't exist,
    /// the operation is still considered successful.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be deleted
    ///
    /// # Returns
    /// * `Ok(())` - The operation was successful (whether or not the key existed)
    /// * `Err(StorageError)` - If an error occurred during deletion
    ///
    async fn atomic_delete(&self, key: String) -> Result<(), crate::errors::StorageError> {
        self.atomic_store.remove(&key);
        Ok(())
    }

    /// Increments an atomic integer value.
    ///
    /// If the key already exists, the value is updated.
    /// The function returns the previous value before the increment.
    ///
    /// # Arguments
    ///
    /// * `key` - The key under which to store the value
    /// * `value` - The integer value to increment by
    ///
    /// # Returns
    ///
    /// * `Ok(Some(i64))` - The previous value before the increment
    /// * `Ok(None)` - If the key doesn't exist
    async fn atomic_increment(
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