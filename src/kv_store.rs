
use std::collections::HashMap;


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

    /// To write a value to a key in store
    /// NOTE: It overwrites old key
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// To retrieve the value of a key from the store
    /// Returns None for non-existent key
    pub fn get(&self, key: String) ->  Option<String> {
        Some(self.store.get(&key)?.to_string())
    }

    /// To remove a key, and it's corresponding value from store
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
