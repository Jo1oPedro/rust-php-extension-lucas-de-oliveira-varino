use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use ext_php_rs::exception::PhpException;

#[php_class]
#[php(name = "Varinha\\Array")]
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

    pub fn array_type(&self) -> String {
        self.array_type.clone()
    }

    pub fn size(&self) -> i64 {
        self.elements.len() as i64
    }

    pub fn adicionar(&mut self, valor: &Zval) -> PhpResult<()> {
        if self.elements.len() >= self.capacity {
            return Err(PhpException::default(
                "array is at full capacity".into(),
            ));
        }

        if valor.get_type().to_string() != self.array_type {
            return Err(PhpException::default(
                format!(
                    "expected type '{}', got '{}'",
                    self.array_type,
                    valor.get_type()
                )
            ))
        }

        self.elements.push(valor.shallow_clone());

        Ok(())
    }

    pub fn get_all(&mut self) -> Vec<Zval> {
        self.elements
            .iter()
            .map(|z| z.shallow_clone())
            .collect()
    }
}