use std::collections::HashMap;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use crate::common::zvals_to_array;

#[php_class]
#[php(name = "Varinha\\VarinhaHashMap")]
pub struct VarinhaHashMap {
    elements: HashMap<String, Zval>
}

#[php_impl]
impl VarinhaHashMap {
    pub fn __construct(initial_values: Option<&ZendHashTable>) -> PhpResult<Self> {
        let mut elements: HashMap<String, Zval> = HashMap::new();

        if let Some(hash_table) = initial_values {
            for (key, value) in hash_table {
                elements.insert(key.to_string(), value.shallow_clone());
            }
        }

        Ok(VarinhaHashMap { elements })
    }

    pub fn has(&self, key: String) -> PhpResult<bool> { Ok(self.elements.contains_key(&key)) }

    pub fn size(&self) -> PhpResult<u64> { Ok(self.elements.len() as u64) }

    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn keys(&self) -> Vec<String> { self.elements.keys().cloned().collect() }

    pub fn values(&self) -> Vec<Zval> { self.elements.values().map(|value| value.shallow_clone()).collect() }

    pub fn to_array(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut hash_table = ZendHashTable::new();

        for (key, value) in &self.elements {
            hash_table.insert(key.as_str(), value.shallow_clone())?;
        }

        Ok(hash_table)
    }

    pub fn put(&mut self, key: String, value: &Zval) -> PhpResult<i64> {
        self.elements.insert(key, value.shallow_clone());

        Ok(self.elements.len() as i64)
    }

    pub fn remove(&mut self, key: String) -> PhpResult<Zval> {
        self.elements
            .remove(&key)
            .ok_or_else(|| PhpException::default("Key not found".into()))
    }

    pub fn clean(&mut self) -> PhpResult<()> {
        Ok(self.elements.clear())
    }

    pub fn get(&self, key: String) -> PhpResult<Zval> {
        self.elements.get(&key)
            .map(|e| e.shallow_clone())
            .ok_or_else(|| PhpException::default("Key not found".into()))
    }

    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        zvals_to_array(self.elements.values().map(|e| e.shallow_clone()))
    }
}