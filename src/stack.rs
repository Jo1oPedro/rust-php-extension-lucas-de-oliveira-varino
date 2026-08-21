use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use crate::common::zvals_to_array;

#[php_class]
#[php(name = "Ds\\DSStack")]
pub struct Stack {
    elements: Vec<Zval>,
}

#[php_impl]
impl Stack {
    pub fn __construct(initial_values: Option<&ZendHashTable>) -> PhpResult<Self> {
        let mut elements: Vec<Zval> = Vec::new();

        if let Some(hash_table) = initial_values {
            for value in hash_table.values() {
                elements.push(value.shallow_clone());
            }
        }

        Ok(Stack { elements })
    }

    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn size(&self) -> i64 { self.elements.len() as i64 }

    pub fn clean(&mut self) -> PhpResult<()> { Ok(self.elements.clear()) }

    pub fn push(&mut self, value: &Zval) {
        self.elements.push(value.shallow_clone())
    }

    pub fn pop(&mut self) -> PhpResult<Zval> {
        if self.is_empty() {
            return Err(PhpException::default(
                "stack is empty".into()
            ));
        }

        Ok(self.elements.pop().unwrap())
    }

    pub fn peek(&self) -> PhpResult<Zval> {
        if self.is_empty() {
            return Err(PhpException::default(
                "stack is empty".into()
            ));
        }

        Ok(self.elements[self.elements.len() - 1].shallow_clone())
    }
    
    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        zvals_to_array(self.elements.iter().map(|e| e.shallow_clone()))
    }
}