use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Zval};
use ext_php_rs::exception::PhpException;
use ext_php_rs::flags::DataType;

#[php_class]
#[php(name = "Varinha\\VarinhaArray")]
pub struct Array {
    elements: Vec<Zval>,
    array_type: String,
    capacity: usize
}

#[php_impl]
impl Array {
    pub fn __construct(capacity: i64, array_type: String) -> PhpResult<Self> {
        if capacity < 0 {
            return Err(
                PhpException::default("size can not be negative".into())
            );
        }

        Ok(Array {
            elements: Vec::with_capacity(capacity as usize),
            array_type,
            capacity: capacity as usize
        })
    }

    // Metadata
    pub fn array_type(&self) -> String { self.array_type.clone() }

    pub fn size(&self) -> i64 { self.elements.len() as i64 }

    pub fn capacity(&self) -> i64 { self.capacity as i64 }

    pub fn is_empty(&self) -> bool { self.elements.is_empty() }

    pub fn is_full(&self) -> bool { self.elements.len() == self.capacity }

    // Access
    pub fn get(&self, index: i64) -> PhpResult<Zval> {
        self.check_position(index)?;

        Ok(self.elements[index as usize].shallow_clone())
    }

    pub fn first(&self) -> PhpResult<Zval> {
        if self.is_empty() {
            return Err(PhpException::default(
                "array is empty".into()
            ));
        }
        Ok(self.get(0)?.shallow_clone())
    }

    pub fn last(&self) -> PhpResult<Zval> {
        if self.is_empty() {
            return Err(PhpException::default(
                "array is empty".into()
            ))
        }
        Ok(self.get(self.size() - 1)?.shallow_clone())
    }

    pub fn get_all(&self) -> PhpResult<Vec<Zval>> {
        Ok(self.elements
            .iter()
            .map(|e| e.shallow_clone())
            .collect())
    }

    // Mutation
    pub fn add(&mut self, value: &Zval) -> PhpResult<i64> {
        if self.elements.len() >= self.capacity {
            return Err(PhpException::default(
                "array is at full capacity".into(),
            ));
        }

        self.check_type(value)?;

        self.elements.push(value.shallow_clone());

        Ok(self.elements.len() as i64)
    }

    pub fn set(&mut self, index: i64, value: &Zval) -> PhpResult<i64> {
        self.check_position(index)?;

        self.check_type(value)?;

        self.elements[index as usize] = value.shallow_clone();

        Ok(self.elements.len() as i64)
    }

    pub fn insert(&mut self, index: i64, value: &Zval) -> PhpResult<i64> {
        if self.elements.len() >= self.capacity {
            return Err(PhpException::default("array is at full capacity".into()));
        }

        self.check_position(index)?;

        self.check_type(value)?;

        self.elements.insert(index as usize, value.shallow_clone());

        Ok((self.elements.len() - 1) as i64)
    }

    pub fn remove(&mut self, index: i64) -> PhpResult<Zval> {
        self.check_position(index)?;

        Ok(self.elements.remove(index as usize))
    }

    pub fn pop(&mut self) -> PhpResult<Zval> {
        self.elements.pop().ok_or_else(|| PhpException::default("array is empty".into()))
    }

    pub fn clean(&mut self) -> PhpResult<()> {
        Ok(self.elements.clear())
    }

    //Search
    pub fn search(&self, needle: &Zval) -> PhpResult<Zval> {
        self.elements
            .iter()
            .find(|element| element.is_identical(needle))
            .map(|element| element.shallow_clone())
            .ok_or_else(|| PhpException::default("Searched value not found on array".into()))
    }

    pub fn index_of(&self, needle: &Zval) -> PhpResult<i64> {
        self.elements
            .iter()
            .position(|element| element.is_identical(needle))
            .map(|index| index as i64)
            .ok_or_else(|| PhpException::default("Index of needle not found on array".into()))
    }

    pub fn __debug_info(&self) -> PhpResult<ZBox<ZendHashTable>> {
        let mut hash_table: ZBox<ZendHashTable> = ZendHashTable::new();
        for element in &self.elements {
            hash_table.push(element.shallow_clone());
        }

        Ok(hash_table)
    }
}

impl Array {
    fn check_type(&self, value: &Zval) -> PhpResult<()> {
        let type_received = match value.object() {
            Some(obj) => obj.get_class_name()?,
            None => php_type_name(value.get_type()),
        };

        if type_received != self.array_type {
            return Err(PhpException::default(format!(
                "expected type '{}', got '{}'",
                self.array_type,
                type_received
            )));
        }

        Ok(())
    }

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

fn php_type_name(dt: DataType) -> String {
    match dt {
        DataType::Long => "int",
        DataType::Double => "float",
        DataType::String => "string",
        DataType::Bool => "bool",
        DataType::True => "bool",
        DataType::False => "bool",
        DataType::Array => "array",
        DataType::Null => "null",
        DataType::Object(_) => "object",
        DataType::Callable => "callable",
        DataType::Resource => "resource",
        DataType::Reference => "reference",
        _ => "mixed",
    }.to_string()
}