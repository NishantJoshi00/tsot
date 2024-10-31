use async_trait::async_trait;
use fred::{
    interfaces::KeysInterface,
    types::{Expiration, RedisKey},
};

use crate::{
    asynchronous::{
        AtomicStorage, RawStorage, RawStorageWithExpiry, StringStorage, StringStorageWithExpiry,
    },
    errors::StorageError,
    types::StoreState,
};

use super::RedisStorageModule;

#[async_trait]
impl StringStorageWithExpiry for RedisStorageModule {
    /// Store a string with optional expiry
    async fn store_with_expiry(
        &self,
        key: String,
        value: String,
        expiry: Option<u64>,
    ) -> Result<StoreState, StorageError> {
        let key = RedisKey::from(key);

        // Determine expiration
        let expiration = expiry.map(|seconds| Expiration::EX(seconds as i64));

        // Check if key exists before setting
        let exists = self
            .client
            .exists(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Set the value with optional expiration
        self.client
            .set::<String, _, _>(key, value, expiration, None, false)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Return store state based on previous existence
        Ok(if exists {
            StoreState::Updated
        } else {
            StoreState::New
        })
    }
}

#[async_trait]
impl StringStorage for RedisStorageModule {
    /// Load a string value
    async fn load_string(&self, key: String) -> Result<Option<String>, StorageError> {
        let key = RedisKey::from(key);

        // Get the value from Redis
        let result: Option<String> = self
            .client
            .get(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        Ok(result)
    }

    /// Delete a string value
    async fn delete_string(&self, key: String) -> Result<(), StorageError> {
        let key = RedisKey::from(key);

        // Delete the key
        self.client
            .del::<u64, _>(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl RawStorageWithExpiry for RedisStorageModule {
    /// Store binary data with optional expiry
    async fn store_raw_with_expiry(
        &self,
        key: String,
        value: Vec<u8>,
        expiry: Option<u64>,
    ) -> Result<StoreState, StorageError> {
        let key = RedisKey::from(key);

        // Determine expiration
        let expiration = expiry.map(|seconds| Expiration::EX(seconds as i64));

        // Check if key exists before setting
        let exists = self
            .client
            .exists(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Set the value with optional expiration
        self.client
            .set::<String, _, _>(key, value, expiration, None, false)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Return store state based on previous existence
        Ok(if exists {
            StoreState::Updated
        } else {
            StoreState::New
        })
    }
}

#[async_trait]
impl RawStorage for RedisStorageModule {
    /// Load binary data
    async fn load_raw(&self, key: String) -> Result<Option<Vec<u8>>, StorageError> {
        let key = RedisKey::from(key);

        // Get the value from Redis
        let result: Option<Vec<u8>> = self
            .client
            .get(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        Ok(result)
    }

    /// Delete binary data
    async fn delete_raw(&self, key: String) -> Result<(), StorageError> {
        let key = RedisKey::from(key);

        // Delete the key
        self.client
            .del::<u64, _>(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl AtomicStorage for RedisStorageModule {
    /// Store an atomic integer value
    async fn atomic_store(&self, key: String, value: i64) -> Result<StoreState, StorageError> {
        let key = RedisKey::from(key);

        // Check if key exists before setting
        let exists = self
            .client
            .exists(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Set the value
        self.client
            .set::<String, _, _>(key, value.to_string(), None, None, false)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Return store state based on previous existence
        Ok(if exists {
            StoreState::Updated
        } else {
            StoreState::New
        })
    }

    /// Load an atomic integer value
    async fn atomic_load(&self, key: String) -> Result<Option<i64>, StorageError> {
        let key = RedisKey::from(key);

        // Get the value from Redis
        let result: Option<String> = self
            .client
            .get(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        // Parse the value to i64
        let parsed_result = result
            .map(|s| {
                s.parse::<i64>()
                    .map_err(|_| StorageError::DeserializationError("Invalid integer".to_string()))
            })
            .transpose()?;

        Ok(parsed_result)
    }

    /// Delete an atomic integer value
    async fn atomic_delete(&self, key: String) -> Result<(), StorageError> {
        let key = RedisKey::from(key);

        // Delete the key
        self.client
            .del::<u64, _>(&key)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        Ok(())
    }

    /// Increment an atomic integer value
    async fn atomic_increment(&self, key: String, value: i64) -> Result<Option<i64>, StorageError> {
        let key = RedisKey::from(key);

        // Use Redis INCRBY command to increment
        let result = self
            .client
            .incr_by(&key, value)
            .await
            .map_err(|e| StorageError::ConnectionError(e.to_string()))?;

        Ok(Some(result))
    }
}
