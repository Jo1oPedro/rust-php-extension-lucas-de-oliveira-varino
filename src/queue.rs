use std::collections::VecDeque;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};

#[php_class]
#[php(name = "Varinha\\VarinhaQueue")]
pub struct Queue {
    elements: VecDeque<Zval>,
}

#[php_impl]
impl Queue {
    pub fn __construct(initial_values: Option<&ZendHashTable>) -> PhpResult<Self> {
        let mut elements: VecDeque<Zval> = VecDeque::new();
        if let Some(hash_table) = initial_values {
            for value in hash_table.values() {
                elements.push_back(value.shallow_clone());
            }
        }

        Ok( Queue { elements } )
    }

    pub fn size(&self) -> i64 {
        self.elements.len() as i64
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clean(&mut self) -> PhpResult<()> {
        Ok(self.elements.clear())
    }

    pub fn enqueue(&mut self, value: &Zval) -> PhpResult<i64> {
        self.elements.push_back(value.shallow_clone());

        Ok(self.elements.len() as i64)
    }

    pub fn dequeue(&mut self) -> PhpResult<Zval> {
    self.elements
            .pop_front()
            .ok_or_else(|| PhpException::default("queue is empty".into()))
    }

    pub fn peek(&self) -> PhpResult<Zval> {
        self.elements
            .front()
            .map(|value| value.shallow_clone())
            .ok_or_else(|| PhpException::default("queue is empty".into()))
    }
}