
use std::collections::HashMap;
use std::path::Path;
use crate::error::Result;

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
    store: HashMap<String, String>
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
    pub fn open(file_path: &Path) -> Result<KvStore> {
        unimplemented!()
    }

    /// To write a value to a key in store
    /// NOTE: It overwrites old key
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
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
