use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use crate::common::zvals_to_array;

#[php_class]
#[php(name = "Varinha\\VarinhaVector")]
pub struct Vector {
    elements: Vec<Zval>
}

#[php_impl]
impl Vector {
    pub fn __construct(initial_values: Option<&ZendHashTable>) -> PhpResult<Self> {
        let mut elements: Vec<Zval> = Vec::new();
        
        if let Some(hash_table) = initial_values {
            for value in hash_table.values() {
                elements.push(value.shallow_clone());
            }
        }
        
        Ok(Vector { elements })
    }

    pub fn size(&self) -> i64 {
        self.elements.len() as i64
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn get(&self, index: i64) -> PhpResult<Zval> {
        self.check_position(index)?;

        Ok(self.elements[index as usize].shallow_clone())
    }

    pub fn first(&self) -> PhpResult<Zval> {
        if self.is_empty() {
            return Err(PhpException::default(
                "Vector is empty".into()
            ));
        }

        Ok(self.elements[0 as usize].shallow_clone())
    }

    pub fn last(&self) -> PhpResult<Zval> {
        if self.is_empty() {
            return Err(PhpException::default(
                "Vector is empty".into()
            ));
        }

        Ok(self.elements[self.size() as usize - 1 as usize].shallow_clone())
    }

    pub fn get_all(&self) -> PhpResult<Vec<Zval>> {
        Ok(
            self.elements
                .iter()
                .map(|e| e.shallow_clone())
                .collect()
        )
    }

    pub fn add(&mut self, value: &Zval) -> i64 {
        self.elements.push(value.shallow_clone());

        self.elements.len() as i64
    }

    pub fn set(&mut self, index: i64, value: &Zval) -> PhpResult<i64> {
        self.check_position(index)?;

        self.elements.insert(index as usize, value.shallow_clone());

        Ok(self.elements.len() as i64)
    }

    pub fn remove(&mut self, index: i64) -> PhpResult<Zval> {
        self.check_position(index)?;

        Ok(self.elements.remove(index as usize))
    }

    pub fn pop(&mut self) -> PhpResult<Zval> {
        self.elements.pop().ok_or_else(|| PhpException::default("Vector is empty".into()))
    }

    pub fn clean(&mut self) -> PhpResult<()> {
        Ok(self.elements.clear())
    }

    pub fn reserve(&mut self, additional: i64) -> PhpResult<()> {
        self.elements.reserve(additional as usize);

        Ok(())
    }

    pub fn shrink_to_fit(&mut self) -> PhpResult<()> {
        self.elements.shrink_to_fit();

        Ok(())
    }

    pub fn extend(&mut self, values: Vec<&Zval>) -> PhpResult<i64> {
        for value in values {
            self.elements.push(value.shallow_clone());
        }

        Ok(self.elements.len() as i64)
    }

    pub fn search(&self, value: &Zval) -> PhpResult<Zval> {
        self.elements
            .iter()
            .find(|e| e.is_identical(value))
            .map(|e| e.shallow_clone())
            .ok_or_else(|| PhpException::default("Searched value not found on vector".into()))
    }

    pub fn index_of(&self, value: &Zval) -> PhpResult<i64> {
        self.elements
            .iter()
            .position(|e| e.is_identical(value))
            .map(|i| i as i64)
            .ok_or_else(|| PhpException::default("Searched value not found on vector".into()))
    }

    pub fn contains(&self, value: &Zval) -> bool {
        self.elements.iter().any(|e| e.is_identical(value))
    }

    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        zvals_to_array(self.elements.iter().map(|e| e.shallow_clone()))
    }
}

impl Vector {
    fn check_position(&self, index: i64) -> PhpResult<()> {
        if index < 0 || index as usize >= self.elements.len() {
            return Err(PhpException::default(
                format!(
                    "Index {} out of bounds (size {})",
                    index,
                    self.elements.len()
                ))
            );
        }

        Ok(())
    }
}

