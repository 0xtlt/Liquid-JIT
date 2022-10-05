use std::collections::HashMap;

use super::types::LiquidDataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataManipulationFunction {
    Replace,
    Assign,
    // Echo,
    NotDefined,
    Plus,
    Minus,
    T,
    Upcase,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataManipulationMode {
    Assign,
    Echo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataManipulationArgsType {
    Arg(LiquidDataType),
    ArgMap(String, LiquidDataType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataManipulation {
    pub(crate) function: DataManipulationFunction,
    pub(crate) args: Vec<LiquidDataType>,
    pub(crate) arg_map: HashMap<String, LiquidDataType>,
}
