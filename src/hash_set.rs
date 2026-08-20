use std::collections::HashSet;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::php_class;
use ext_php_rs::types::ZendHashTable;

#[php_class]
#[php(name = "Varinha\\VarinhaHashSet")]
pub struct VarinhaHashSet {
    elements: HashSet<String>,
}

#[php_impl]
impl VarinhaHashSet {
    pub fn __construct(initial_values: Option<&ZendHashTable>) -> PhpResult<Self> {
        let mut elements: HashSet<String> = HashSet::new();

        if let Some(hash_table) = initial_values {
            for value in hash_table.values() {
                let final_value = value.string().ok_or_else(|| PhpException::default("all values must be strings".into()))?;
                elements.insert(final_value);
            }
        }

        Ok(VarinhaHashSet { elements, })
    }

    pub fn contains(&self, value: String) -> bool { self.elements.contains(&value) }

    pub fn size(&self) -> usize { self.elements.len() }

    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn add(&mut self, value: String) -> bool { self.elements.insert(value) }

    pub fn remove(&mut self, value: String) -> bool { self.elements.remove(&value) }

    pub fn clean(&mut self) -> () { self.elements.clear() }

    pub fn union(&self, other: &VarinhaHashSet) -> VarinhaHashSet {
        let elements = self.elements
            .union(&other.elements)
            .cloned()
            .collect();

        VarinhaHashSet { elements }
    }

    pub fn intersection(&self, other: &VarinhaHashSet) -> VarinhaHashSet {
        let elements = self.elements
            .intersection(&other.elements)
            .cloned()
            .collect();

        VarinhaHashSet { elements }
    }

    pub fn difference(&self, other: &VarinhaHashSet) -> VarinhaHashSet {
        let elements = self.elements
            .difference(&other.elements)
            .cloned()
            .collect();

        VarinhaHashSet { elements }
    }

    pub fn is_subset(&self, other: &VarinhaHashSet) -> bool { self.elements.is_subset(&other.elements) }

    pub fn to_array(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut hash_table = ZendHashTable::new();

        for value in &self.elements {
            hash_table.push(value.as_str())?;
        }

        Ok(hash_table)
    }

    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        self.to_array()
    }
}