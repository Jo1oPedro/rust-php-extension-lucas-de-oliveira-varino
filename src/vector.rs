use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use ext_php_rs::exception::PhpException;

#[php_class]
#[php(name = "Varinha\\Vector")]
pub struct Vector {
    elements: Vec<Zval>,
    vector_type: String,
    size: usize
}

#[php_impl]
impl Vector {
    pub fn __construct(size: i64, vector_type: String) -> PhpResult<Self> {
        if size < 0 {
            return Err(
                PhpException::default("size can not be negative".into())
            );
        }

        Ok(Vector {
            elements: Vec::with_capacity(size as usize),
            vector_type: vector_type,
            size: size as usize
        })
    }

    pub fn vector_type(&self) -> String {
        self.vector_type.clone()
    }

    pub fn size(&self) -> i64 {
        self.elements.len() as i64
    }

    /*pub fn adicionar(&mut self, valor: &Zval) -> DataType {
        valor.get_type()
    }*/

    pub fn teste(&self) -> String {
        String::from("oi")
    }
}