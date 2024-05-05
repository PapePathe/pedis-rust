use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display};

pub trait IStore {
    fn set(&mut self, k: String, v: Value) -> Result<(), KeyNotFoundError>; 
    fn get(&self, k: String, vk: ValueKind) -> Result<&Value, KeyNotFoundError>; 
}


#[derive(Default)]
pub struct RedisStore {
    store: HashMap<String, Value>,
}
impl IStore for RedisStore {
    fn set(&mut self, k: String, v: Value) -> Result<(), KeyNotFoundError> {
        self.store.insert(k, v);
        Ok(())
    }
    fn get(&self, k: String, vk: ValueKind) -> Result<&Value, KeyNotFoundError> {
        match self.store.get(&k.clone()) {
            Some(value) => {
                if value.kind == vk {
                    return Ok(value);
                }

                Err(KeyNotFoundError {})
            }
            None => Err(KeyNotFoundError {}),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct KeyNotFoundError {}
impl Error for KeyNotFoundError {}
impl Display for KeyNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STORE ERROR: err msg")
    }
}

#[cfg(test)]
mod test {
    use super::{RedisStore, Value, ValueKind, KeyNotFoundError};
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
        assert_eq!(Err(KeyNotFoundError{}), get_key_kind_mistmatch_result);

        let get_key_not_found_result = s.get("key:013".to_string(), ValueKind::String);
        assert_eq!(Err(KeyNotFoundError {}), get_key_not_found_result);
    }
}

#[derive(PartialEq)]
pub struct Value {
    pub kind: ValueKind,
    pub data: Vec<u8>
    //    created_at: u64,
    //    last_read_at: u64,
    //    updated_at: u64,
    //    expires_at: u64
}

impl Value {
    pub fn new(data: Vec<u8>, kind: ValueKind) -> Self { 
        Self{kind, data}
    }
    pub fn new_string(data: Vec<u8>) -> Self {
        Self{kind: ValueKind::String, data}
    }
    pub fn new_map(data: Vec<u8>) -> Self {
        Self{kind: ValueKind::Map, data}
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "k={:} len={:}", self.kind, self.data.len())
    }
}

#[derive(PartialEq)]
pub enum ValueKind {
    String,
    Map,
    Json,
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
