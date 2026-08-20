use ext_php_rs::boxed::ZBox;
use ext_php_rs::prelude::PhpResult;
use ext_php_rs::types::{ZendHashTable, Zval};

pub fn zvals_to_array(
    values: impl Iterator<Item = Zval>
) -> PhpResult<ZBox<ZendHashTable>> {
    let mut hash_table = ZendHashTable::new();
    
    for element in values {
        hash_table.push(element)?;
    }
    
    Ok(hash_table)
}