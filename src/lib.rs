pub mod array;
pub mod vector;
pub mod stack;
pub mod queue;
pub mod hash_map;
pub mod hash_set;
pub mod priority_queue;
pub mod common;
pub mod btree_map;

use ext_php_rs::prelude::*;
use crate::priority_queue::PhpInterfacePrioritizable;

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<array::Array>()
        .class::<vector::Vector>()
        .class::<queue::Queue>()
        .class::<hash_map::DSHashMap>()
        .class::<hash_set::DSHashSet>()
        .class::<priority_queue::DSPriorityQueue>()
        .interface::<PhpInterfacePrioritizable>()
        .class::<btree_map::DSBTreeMap>()
}
