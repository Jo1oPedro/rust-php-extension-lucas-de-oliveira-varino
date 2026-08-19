use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
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
        if index < 0 || index as usize >= self.elements.len() {
            return Err(PhpException::default(
               format!(
                   "Index {} out of bounds (size {})",
                   index,
                   self.elements.len()
               )
            ));
        }

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
    pub fn add(&mut self, valor: &Zval) -> PhpResult<i64> {
        if self.elements.len() >= self.capacity {
            return Err(PhpException::default(
                "array is at full capacity".into(),
            ));
        }

        let tipo_recebido = match valor.object() {
            Some(obj) => obj.get_class_name()?,
            None => php_type_name(valor.get_type()),
        };

        if tipo_recebido != self.array_type {
            return Err(PhpException::default(
                format!(
                    "expected type '{}', got '{}'",
                    self.array_type,
                    tipo_recebido
                )
            ))
        }

        self.elements.push(valor.shallow_clone());

        Ok(self.elements.len() as i64)
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