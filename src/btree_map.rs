use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};
use ext_php_rs::{php_class, php_impl};
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};

#[php_class]
#[php(name = "Varinha\\VarinhaBTreeMap")]
pub struct VarinhaBTreeMap {
    elements: BTreeMap<String, Zval>
}

#[php_impl]
impl VarinhaBTreeMap {
    pub fn __construct(initial_values: Option<&ZendHashTable>) -> PhpResult<Self> {
        let mut elements: BTreeMap<String, Zval> = BTreeMap::new();

        if let Some(hash_table) = initial_values {
            for (key, value) in hash_table.iter() {
                elements.insert(key.to_string(), value.shallow_clone());
            }
        }

        Ok(VarinhaBTreeMap { elements })
    }

    pub fn put(&mut self, key: String, value: &Zval) -> PhpResult<()> {
        self.elements.insert(key, value.shallow_clone());

        Ok(())
    }

    pub fn remove(&mut self, key: String) -> PhpResult<Zval> {
        self.elements
            .remove(&key)
            .ok_or_else(|| PhpException::default("Key not found".into()))
    }

    pub fn clean(&mut self) -> PhpResult<()> {
        self.elements.clear();

        Ok(())
    }

    pub fn get(&self, key: String) -> PhpResult<Zval> {
        self.elements
            .get(&key)
            .map(|e| e.shallow_clone())
            .ok_or_else(|| PhpException::default("Key not found".into()))
    }

    pub fn has(&self, key: String) -> bool { self.elements.contains_key(&key) }

    pub fn size(&self) -> i64 { self.elements.len() as i64 }

    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn first_key(&self) -> PhpResult<String> {
        self.elements
            .first_key_value()
            .map(|(k, _)| k.clone())
            .ok_or_else(|| PhpException::default("BTree map is empty".into()))
    }

    pub fn last_key(&self) -> PhpResult<String> {
        self.elements
            .last_key_value()
            .map(|(k, _)| k.clone())
            .ok_or_else(|| PhpException::default("BTree map is empty".into()))
    }

    pub fn first(&self) -> PhpResult<Zval> {
        self.elements
            .first_key_value()
            .map(|(_, k)| k.shallow_clone())
            .ok_or_else(|| PhpException::default("BTree map is empty".into()))
    }

    pub fn last(&self) -> PhpResult<Zval> {
        self.elements
            .last_key_value()
            .map(|(_, k)| k.shallow_clone())
            .ok_or_else(|| PhpException::default("BTree map is empty".into()))
    }

    pub fn range(&self, from: Option<String>, to: Option<String>) -> PhpResult<ZBox<ZendHashTable>> {
        let mut hash_table = ZendHashTable::new();

        let start = match from {
            Some(f) => Included(f),
            None => Unbounded
        };

        let end = match to {
            Some(f) => Included(f),
            None => Unbounded
        };

        for (key, value) in self.elements.range((start, end)) {
            hash_table.insert(key.clone(), value.shallow_clone())?;
        }

        Ok(hash_table)
    }

    pub fn to_array(&self) -> PhpResult<ZBox<ZendHashTable>> { self.range(None, None) }

    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        self.to_array()
    }
}