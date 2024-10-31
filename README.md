<div align="center">
    <h1>TSOT</h1>
    <p>That Storage Over There</p>
    <img alt="Rust" src="https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square&logo=rust"/>
    <img alt="License" src="https://img.shields.io/github/license/nishantjoshi00/tsot?style=flat-square"/>
    <img alt="Build Status" src="https://img.shields.io/github/actions/workflow/status/nishantjoshi00/tsot/rust.yml?style=flat-square"/>
    <img alt="Crates.io" src="https://img.shields.io/crates/v/tsot?style=flat-square"/>
</div>

## 📖 Overview

TSOT (That Storage Over There) is a versatile, high-performance storage abstraction library for Rust, designed to provide a unified interface across multiple storage backends.

## ✨ Features

- 🔌 **Pluggable Backends**: Seamlessly switch between different storage systems
- 🚀 **Multi-Type Support**: Store strings, raw bytes, and atomic integers
- ⏰ **Flexible Expiration**: Time-based item expiration
- 🔀 **Async & Sync**: First-class support for both asynchronous and synchronous code
- 🔒 **Thread-Safe**: Concurrent access without compromising performance

## 🛠 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tsot = "0.1.0"

# Optional: Enable specific backends
tsot = { version = "0.1.0", features = ["imc", "async"] }
```

## 💡 Quick Start

### Basic Usage

```rust
use tsot::{Storage, StorageResult};

// Generic storage interface
let storage = Storage::new_imc();

// Store and retrieve
storage.store_string("key", "value")?;
let value = storage.load_string("key")?;
```

### Async Operations

```rust
use tsot::AsyncStorage;

async fn example() {
    let storage = AsyncStorage::new_imc();
    storage.store_raw("bytes_key", vec![1, 2, 3]).await?;
}
```

## 🗃 Supported Backends

| Backend   | Status          | Async | Sync |
| --------- | --------------- | ----- | ---- |
| In-Memory | ✅ Stable       | ✅    | ✅   |
| Redis     | ⚠️ Experimental | ✅    | ❌   |
| Memcached | 🚧 Planned      | ✅    | ✅   |
| RocksDB   | 🚧 Planned      | ✅    | ✅   |

## 📊 Benchmarks

Performance is a key priority. Detailed benchmarks coming soon!

## 🛤 Roadmap

- [x] In-Memory Backend
- [ ] Redis Backend
- [ ] Memcached Backend
- [ ] Performance Benchmarking
- [ ] Persistent Storage Support

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Quick Contribution Steps

1. Fork the repository
2. Create a feature branch
3. Implement your backend or feature
4. Write tests
5. Submit a pull request

## 📄 License

Licensed under MIT License. See [LICENSE](LICENSE) for more details.

## 🌟 Acknowledgments

Built with ❤️ using Rust's powerful type system and concurrency primitives.
