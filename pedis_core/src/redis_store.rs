use std::collections::HashMap;
use std::fmt::{Debug, Display};

/// Store errors
#[derive(Debug, PartialEq)]
pub enum StoreError {
    /// Error returned when a key was not found on the store
    KeyNotFoundError,
    /// Error raised when trying to access a key but the kind of data does not match
    KeyMismatchError(String),
}

impl Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyNotFoundError => {
                write!(f, "-ERR key not found")
            }
            Self::KeyMismatchError(m) => {
                write!(f, "-ERR {:}", m)
            }
        }
    }
}

type StoreResult<T> = Result<T, StoreError>;

/// Defines the storage interface
pub trait IStore {
    /// Set given value using specified key
    fn set(&mut self, k: String, v: Value) -> StoreResult<()>;
    /// Retrieves a key from the store
    fn get(&self, k: String, vk: ValueKind) -> StoreResult<&Value>;
}

/// Default implementation of the store trait
#[derive(Default)]
pub struct RedisStore {
    store: HashMap<String, Value>,
}
impl IStore for RedisStore {
    fn set(&mut self, k: String, v: Value) -> StoreResult<()> {
        self.store.insert(k, v);
        Ok(())
    }
    fn get(&self, k: String, vk: ValueKind) -> StoreResult<&Value> {
        match self.store.get(&k.clone()) {
            Some(value) => {
                if value.kind == vk {
                    return Ok(value);
                }

                Err(StoreError::KeyMismatchError(
                    "key xxx does not match yyy".to_string(),
                ))
            }
            None => Err(StoreError::KeyNotFoundError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{RedisStore, StoreError, Value, ValueKind};
    use crate::redis_store::IStore;

    #[test]
    fn test_store_get_set() {
        let mut s = RedisStore::default();
        let value = Value::new_string("hello pedis".to_string().as_bytes().to_vec());
        let set_result = s.set("key:001".to_string(), value);
        assert_eq!(set_result, Result::Ok(()));

        let expected_value = Value::new_string("hello pedis".to_string().as_bytes().to_vec());
        let get_result = s.get("key:001".to_string(), ValueKind::String);
        assert_eq!(Result::Ok(&expected_value), get_result);

        let get_key_kind_mistmatch_result = s.get("key:001".to_string(), ValueKind::Map);
        assert_eq!(
            Err(StoreError::KeyMismatchError(
                "key xxx does not match yyy".to_string()
            )),
            get_key_kind_mistmatch_result
        );

        let get_key_not_found_result = s.get("key:013".to_string(), ValueKind::String);
        assert_eq!(Err(StoreError::KeyNotFoundError), get_key_not_found_result);
    }
}

/// Represents a value in our storage interface
#[derive(PartialEq)]
pub struct Value {
    /// The kind of data stored in this value
    pub kind: ValueKind,
    /// The data as an array of bytes
    pub data: Vec<u8>, //    created_at: u64,
                       //    last_read_at: u64,
                       //    updated_at: u64,
                       //    expires_at: u64
}

impl Value {
    /// Creates a new value with the desired ValueKind
    pub fn new(data: Vec<u8>, kind: ValueKind) -> Self {
        Self { kind, data }
    }
    /// Create a new value of kind string
    pub fn new_string(data: Vec<u8>) -> Self {
        Self {
            kind: ValueKind::String,
            data,
        }
    }
    /// Create a new value of kind map
    pub fn new_map(data: Vec<u8>) -> Self {
        Self {
            kind: ValueKind::Map,
            data,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "k={:} len={:}", self.kind, self.data.len())
    }
}

/// Represents the kind of values in the pedis store
#[derive(PartialEq)]
pub enum ValueKind {
    /// Used when storing simple strings
    String,
    /// Used when storing data as a map
    Map,
    /// Used when storing json
    Json,
    /// Used when storing lists
    List,
}

impl Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ValueKind::Map => {
                write!(f, "map")
            }
            ValueKind::Json => {
                write!(f, "json")
            }
            ValueKind::List => {
                write!(f, "list")
            }
            ValueKind::String => {
                write!(f, "string")
            }
        }
    }
}

/// Mock store for testing purposes
pub struct Teststore {
    /// Allow the consumer to raise an error while running the tests
    pub err: bool,
}
impl IStore for Teststore {
    fn set(&mut self, _: String, _: Value) -> Result<(), StoreError> {
        if self.err {
            return Err(StoreError::KeyNotFoundError);
        }
        Ok(())
    }
    fn get(
        &self,
        _: String,
        _: crate::redis_store::ValueKind,
    ) -> Result<&crate::redis_store::Value, crate::redis_store::StoreError> {
        todo!()
    }
}
