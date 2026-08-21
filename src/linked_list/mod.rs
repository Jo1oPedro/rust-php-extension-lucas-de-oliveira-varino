use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use crate::linked_list::node::Node;

mod node;

#[php_class]
#[php(name = "Ds\\DSLinkedList")]
pub struct LinkedList {
    head: Option<Box<Node>>
}

#[php_impl]
impl LinkedList {
    pub fn __construct() -> PhpResult<LinkedList> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn push(&mut self, data: &Zval) -> PhpResult<()> {
        todo!()
    }

    pub fn unshift(&mut self, data: &Zval) -> PhpResult<()> {
        todo!()
    }

    pub fn pop(&mut self) -> PhpResult<Zval> {
        todo!()
    }

    pub fn shift(&mut self) -> PhpResult<Zval> {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn to_array(&self) -> PhpResult<Zval> {
        todo!()
    }
}

