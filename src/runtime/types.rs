use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LiquidVariableType {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<LiquidVariableType>),
    Object(HashMap<String, LiquidVariableType>),
    Nil,
}
