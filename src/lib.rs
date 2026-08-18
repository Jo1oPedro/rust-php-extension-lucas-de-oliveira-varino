use ext_php_rs::prelude::*;

#[php_function]
pub fn varinha(mensagem: String) {
    php_println!("Mensagem para o varinha: {}", mensagem);
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(varinha))
}
