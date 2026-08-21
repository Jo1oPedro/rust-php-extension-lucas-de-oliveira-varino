use ext_php_rs::types::Zval;

pub struct Node {
    pub data: Zval,
    pub next: Option<Box<Node>>
}