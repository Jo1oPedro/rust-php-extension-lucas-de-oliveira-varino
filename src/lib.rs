pub mod array;
pub mod vector;
pub mod stack;
pub mod queue;
pub mod hash_map;
pub mod hash_set;
pub mod priority_queue;
pub mod common;

use std::collections::HashMap;
use ext_php_rs::prelude::*;
use crate::priority_queue::PhpInterfacePrioritizable;

#[php_function]
pub fn varinha(mensagem: String) {
    php_println!("Mensagem para o varinha: {}", mensagem);
}

#[php_function]
pub fn call_varinha() -> String {
    reqwest::blocking::get("https://google.com")
        .and_then(|resp| resp.text())
        .unwrap_or_else(|e| format!("erro: {e}"))
}

#[php_function]
pub fn call_varinha_headers() -> HashMap<String, Vec<String>> {
    let mut resultado: HashMap<String, Vec<String>> = HashMap::new();

    match reqwest::blocking::get("https://google.com/headers") {
        Ok(resp) => {
            for(nome, valor) in resp.headers().iter() {
                resultado
                    .entry(nome.to_string())
                    .or_default()
                    .push(valor.to_str().unwrap_or("").to_string());
            }

            resultado
                .entry("status".to_string())
                .or_default()
                .push(resp.status().to_string())
        }
        Err(resp) => {
            resultado
                .entry("status".to_string())
                .or_default()
                .push(resp.to_string());
        }
    }

    resultado
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<array::Array>()
        .class::<vector::Vector>()
        .class::<queue::Queue>()
        .class::<hash_map::VarinhaHashMap>()
        .class::<hash_set::VarinhaHashSet>()
        .class::<priority_queue::VarinhaPriorityQueue>()
        .interface::<PhpInterfacePrioritizable>()
        .function(wrap_function!(varinha))
        .function(wrap_function!(call_varinha))
        .function(wrap_function!(call_varinha_headers))
}
