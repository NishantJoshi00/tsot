# TSOT (That Storage Over There)

TSOT is a flexible, pluggable storage interface designed to provide a unified abstraction for different storage backends across sync and async systems.

## Overview

TSOT aims to provide a universal storage solution with:
- Pluggable backend support
- Consistent interface across different storage systems
- Support for various data types and operations
- Synchronous and asynchronous interfaces

### Key Features

- 🔌 **Pluggable Backends**: Designed to support multiple storage systems
- 🚀 **Flexible Storage**: Store strings, raw bytes, and atomic integers
- ⏰ **Optional Expiration**: Set time-based expiration for stored items
- 🔀 **Async & Sync Support**: Seamless integration with both async and sync codebases
- 🧩 **Trait-Based Design**: Easily extend and implement custom storage backends

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tsot = { version = "0.1.0" }

# Optional: Enable specific backends or features
tsot = { version = "0.1.0", features = ["imc", "redis", "async", "sync"] }
```

## Supported Backends

TSOT is designed with a pluggable architecture to support multiple storage backends:

- [x] In-Memory Cache (IMC)
- [ ] Redis
- [ ] Memcached
- [ ] RocksDB
- [ ] SQLite
- [ ] Etcd
- [ ] DynamoDB
- [ ] S3 / MinIO

## Usage Examples

### Generic Storage Interface

```rust
use tsot::Storage;

// Works across different backends
let storage: Box<dyn Storage> = get_storage_backend();

// Consistent operations regardless of backend
storage.store_string("key", "value")?;
let value = storage.load_string("key")?;
```

### In-Memory Cache Backend

```rust
use tsot::backends::IMCModule;

let cache = IMCModule::new(IMCConfig {});

// Store a string with optional expiration
cache.store_with_expiry("key".to_string(), "value".to_string(), Some(3600))?;
```

### Redis Backend (Future Example)

```rust
use tsot::backends::RedisModule;

let redis_storage = RedisModule::new("redis://localhost:6379")?;

// Same interface, different backend
redis_storage.store_string("key", "value")?;
```

## Supported Operations

TSOT provides a consistent interface across all backends:

- String operations
- Raw byte storage
- Atomic integer handling
- Optional expiration
- Synchronous and asynchronous methods

## Design Philosophy

TSOT is built on the principle of providing a uniform storage abstraction that:
- Minimizes backend-specific code
- Allows easy swapping of storage systems
- Provides a consistent developer experience

## Performance Considerations

- Minimal abstraction overhead
- Backend-specific optimizations
- Trait-based design for zero-cost abstractions

## Limitations

- Abstraction may have slight performance overhead
- Backend-specific features might not be fully supported
- Expiration behavior can vary between backends

## Contributing

We welcome contributions! Help us expand backend support, improve interfaces, and enhance performance.

### Contributing Guidelines

1. Implement a new backend by following the `Storage` trait
2. Add comprehensive tests
3. Ensure performance and compatibility

## Roadmap

- [x] In-Memory Cache Backend
- [ ] Redis Backend
- [ ] Memcached Backend
- [ ] Add more backend support
- [ ] Performance benchmarks
- [ ] Enhanced configuration options
- [ ] Comprehensive documentation

## Acknowledgments

Built with ❤️ using Rust's powerful type system, trait system, and concurrency primitives.

## License

[Your License Here]
