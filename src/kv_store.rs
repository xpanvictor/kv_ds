
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::error::{KvsError, KvsErrorKind, Result};
use serde::{Serialize, Deserialize};



// Since we are using json, we can create command types
#[derive(Debug, Serialize, Deserialize)]
enum KvCommand {
    SetCommand(String, String),
    RmCommand(String)
}

impl KvCommand {
    fn set(key: String, value: String) -> KvCommand {
        KvCommand::SetCommand(key, value)
    }

    fn remove(key: String) -> KvCommand {
        KvCommand::RmCommand(key)
    }
}


/// Key-Value store
/// 
/// Usage sample:
/// 
/// ```rust
/// use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("Key1".to_owned(), "Value1".to_owned());
/// let value = store.get("Key1".to_owned());
/// assert_eq!(Some("Value1".to_owned()), value)
/// ```
pub struct KvStore{
    // old basic impl
    store: HashMap<String, String>,
    // using logs and log pointer
    instance_dir: PathBuf, // A single log file
    key_dir_file: PathBuf,
    // TODO: a map of key to log pointer
    key_dir: HashMap<String, PointerBuf>
}

struct PointerBuf {
    file_path: PathBuf,
    offset: usize,
    value: String,
}


impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}



impl KvStore {
    /// To create a new store
    pub fn new() -> KvStore {
        KvStore{
            store: HashMap::new()
        }
    }

    /// Open KvStore at a file path
    pub fn open(dir_path: &Path) -> Result<KvStore> {
        // Ensure valid dir path is sent
        if !dir_path.is_dir() {
            return Err(KvsErrorKind::GeneralError.into())
        }
        Ok(
            KvStore {
                instance_dir: dir_path.to_owned(),
                store: HashMap::new(),
                key_dir_file: dir_path.join("key_dir.txt"),
                // TODO: read the key dir here
                key_dir: HashMap::new()
            }
        )
    }

    /// To write a value to a key in store
    /// NOTE: It overwrites old key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // Reimplementation, writes to log file instead of memory
        let set_command = KvCommand::set(key, value);
        // json serialize to string
        let set_command_str = serde_json::to_string(&set_command);
        // write to log instead of in memory
        // self.store.insert(key, value);
        Ok(())
    }

    /// To retrieve the value of a key from the store
    /// Returns None for non-existent key
    pub fn get(&self, key: String) ->  Result<Option<String>> {
        let res = self.store.get(&key);
        if res.is_some() {
            Ok(Some(res.unwrap().to_string()))
        } else {
            Ok(None)
        }
    }

    /// To remove a key, and it's corresponding value from store
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }
}
