#![deny(missing_docs)]


//! This is a key-value store
pub use kv_store::KvStore;
pub use error::Result;


mod kv_store;
mod error;


