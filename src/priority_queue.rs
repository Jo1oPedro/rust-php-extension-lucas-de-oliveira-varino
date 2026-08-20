use std::cmp::Ordering;
use std::collections::BinaryHeap;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::php_class;
use ext_php_rs::types::{ZendHashTable, Zval};
use crate::common::zvals_to_array;

#[php_interface]
#[php(name =  "Varinha\\Prioritizable")]
trait Prioritizable {
    fn get_priority(&self) -> f64;
}

struct Entry {
    priority: f64,
    value: Zval
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .partial_cmp(&other.priority)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Entry {}

#[php_class]
#[php(name = "Varinha\\VarinhaPriorityQueue")]
pub struct VarinhaPriorityQueue {
    elements: BinaryHeap<Entry>,
    is_min: bool
}

#[php_impl]
impl VarinhaPriorityQueue {
    const MAX: bool = false;
    const MIN: bool = true;

    pub fn __construct(intial_values: Option<&ZendHashTable>, is_min: Option<bool>) -> PhpResult<Self> {
        let min = is_min.unwrap_or(Self::MAX);
        let mut elements: BinaryHeap<Entry> = BinaryHeap::new();

        if let Some(hash_table) = intial_values {
            for value in hash_table.values() {
                let priority = extract_priority(value)?;

                elements.push(Entry {
                    priority: if min { -priority } else { priority },
                    value: value.shallow_clone()
                })
            }
        }

        Ok(VarinhaPriorityQueue { elements, is_min: min })
    }

    pub fn push(&mut self, value: &Zval) -> PhpResult<i64> {
        let priority = extract_priority(value)?;

        self.elements.push(Entry {
            priority: if self.is_min { -priority } else { priority },
            value: value.shallow_clone()
        });

        Ok(self.elements.len() as i64)
    }

    pub fn peek(&self) -> PhpResult<Zval> {
        self.elements
            .peek()
            .map(|e| e.value.shallow_clone())
            .ok_or_else(|| PhpException::default("No elements available".into()))
    }

    pub fn size(&self) -> i64 { self.elements.len() as i64 }

    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn clean(&mut self) -> () { self.elements.clear() }
    
    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        zvals_to_array(self.elements.iter().map(|e| e.value.shallow_clone()))
    }
}

fn extract_priority(value: &Zval) -> PhpResult<f64> {
    let priority = if let Some(n) = value.long() {
        n as f64
    } else if let Some(f) = value.double() {
        f
    } else if let Some(obj) = value.object() {
        let result = obj
            .try_call_method("getPriority", vec![])
            .map_err(|_| PhpException::default("object must implement Varinha\\Prioritizable".into()))?;

        result.double().ok_or_else(|| PhpException::default(
            "getPriority() must return a float".into()
        ))?
    } else {
        return Err(PhpException::default(
            "value is not prioritizable".into()
        ));
    };

    if priority.is_nan() {
        return Err(PhpException::default("priority cannot be NaN".into()));
    }

    Ok(priority)
}