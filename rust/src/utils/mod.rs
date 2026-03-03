pub mod print_all;

use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AttrValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

pub type AttrMap = HashMap<String, AttrValue>;