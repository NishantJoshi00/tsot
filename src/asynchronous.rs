//! Storage traits for handling different types of data storage operations.
//!
//! This module provides traits for implementing various storage mechanisms with
//! different data types and storage capabilities. It includes support for string storage,
//! raw bytes storage, and atomic numerical operations.

use crate::{errors, types};
use async_trait::async_trait;

/// Trait for basic string storage operations.
///
/// Provides methods for storing, loading, and deleting string values using string keys.
/// Implements [`StringStorageWithExpiry`] and provides a default implementation for
/// storing strings without expiration.
#[async_trait]
pub trait StringStorage: StringStorageWithExpiry {
    /// Stores a string value with the given key.
    ///
    /// This is a convenience method that calls `store_with_expiry` with no expiration time.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The string value to store
    ///
    /// # Returns
    /// * `Result<types::StoreState, errors::StorageError>` - The result of the storage operation
    async fn store_string(
        &self,
        key: String,
        value: String,
    ) -> Result<types::StoreState, errors::StorageError> {
        self.store_with_expiry(key, value, None).await
    }

    /// Loads a string value associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be loaded
    ///
    /// # Returns
    /// * `Result<Option<String>, errors::StorageError>` - The stored string if it exists
    async fn load_string(&self, key: String) -> Result<Option<String>, errors::StorageError>;

    /// Deletes a string value associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be deleted
    ///
    /// # Returns
    /// * `Result<(), errors::StorageError>` - Success or error status of the deletion
    async fn delete_string(&self, key: String) -> Result<(), errors::StorageError>;
}

/// Trait for string storage operations with expiration support.
///
/// Enables storing string values with an optional expiration time.
#[async_trait]
pub trait StringStorageWithExpiry {
    /// Stores a string value with an optional expiration time.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The string value to store
    /// * `expiry` - Optional expiration time in seconds from now
    ///
    /// # Returns
    /// * `Result<types::StoreState, errors::StorageError>` - The result of the storage operation
    async fn store_with_expiry(
        &self,
        key: String,
        value: String,
        expiry: Option<u64>,
    ) -> Result<types::StoreState, errors::StorageError>;
}

/// Trait for basic raw bytes storage operations.
///
/// Provides methods for storing, loading, and deleting raw byte vectors using string keys.
/// Implements [`RawStorageWithExpiry`] and provides a default implementation for
/// storing bytes without expiration.
#[async_trait]
pub trait RawStorage: RawStorageWithExpiry {
    /// Stores raw bytes with the given key.
    ///
    /// This is a convenience method that calls `store_raw_with_expiry` with no expiration time.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The byte vector to store
    ///
    /// # Returns
    /// * `Result<types::StoreState, errors::StorageError>` - The result of the storage operation
    async fn store_raw(
        &self,
        key: String,
        value: Vec<u8>,
    ) -> Result<types::StoreState, errors::StorageError> {
        self.store_raw_with_expiry(key, value, None).await
    }

    /// Loads raw bytes associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be loaded
    ///
    /// # Returns
    /// * `Result<Option<Vec<u8>>, errors::StorageError>` - The stored bytes if they exist
    async fn load_raw(&self, key: String) -> Result<Option<Vec<u8>>, errors::StorageError>;

    /// Deletes raw bytes associated with the given key.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be deleted
    ///
    /// # Returns
    /// * `Result<(), errors::StorageError>` - Success or error status of the deletion
    async fn delete_raw(&self, key: String) -> Result<(), errors::StorageError>;
}

/// Trait for raw bytes storage operations with expiration support.
///
/// Enables storing raw byte vectors with an optional expiration time.
#[async_trait]
pub trait RawStorageWithExpiry {
    /// Stores raw bytes with an optional expiration time.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The byte vector to store
    /// * `expiry` - Optional expiration time in seconds from now
    ///
    /// # Returns
    /// * `Result<types::StoreState, errors::StorageError>` - The result of the storage operation
    async fn store_raw_with_expiry(
        &self,
        key: String,
        value: Vec<u8>,
        expiry: Option<u64>,
    ) -> Result<types::StoreState, errors::StorageError>;
}

/// Trait for atomic operations on integer values.
///
/// Provides methods for storing, loading, deleting, and incrementing integer values
/// in an atomic way, ensuring thread safety and consistency.
#[async_trait]
pub trait AtomicStorage {
    /// Atomically stores an integer value.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The integer value to store
    ///
    /// # Returns
    /// * `Result<types::StoreState, errors::StorageError>` - The result of the storage operation
    async fn atomic_store(
        &self,
        key: String,
        value: i64,
    ) -> Result<types::StoreState, errors::StorageError>;

    /// Atomically loads an integer value.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be loaded
    ///
    /// # Returns
    /// * `Result<Option<i64>, errors::StorageError>` - The stored integer if it exists
    async fn atomic_load(&self, key: String) -> Result<Option<i64>, errors::StorageError>;

    /// Atomically deletes an integer value.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be deleted
    ///
    /// # Returns
    /// * `Result<(), errors::StorageError>` - Success or error status of the deletion
    async fn atomic_delete(&self, key: String) -> Result<(), errors::StorageError>;

    /// Atomically increments an integer value.
    ///
    /// # Arguments
    /// * `key` - The key whose value should be incremented
    /// * `value` - The amount to increment by (can be negative for decrements)
    ///
    /// # Returns
    /// * `Result<Option<i64>, errors::StorageError>` - The new value after incrementing if successful
    async fn atomic_increment(
        &self,
        key: String,
        value: i64,
    ) -> Result<Option<i64>, errors::StorageError>;
}
