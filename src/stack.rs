use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};

#[php_class]
#[php(name = "Varinha\\VarinhaStack")]
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
}