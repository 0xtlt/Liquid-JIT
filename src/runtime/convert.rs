use crate::jit::types::LiquidDataType;

use super::types::LiquidVariableType;

impl LiquidVariableType {
    pub fn convert_to_string(&self) -> String {
        match self {
            LiquidVariableType::String(content) => content.clone(),
            LiquidVariableType::Number(value) => value.to_string(),
            LiquidVariableType::Boolean(value) => value.to_string(),
            LiquidVariableType::Array(_) => panic!("Cannot convert array to string"),
            LiquidVariableType::Object(_) => panic!("Cannot convert object to string"),
            LiquidVariableType::Nil => String::new(),
        }
    }
    pub fn convert_to_number(&self) -> f64 {
        match self {
            LiquidVariableType::String(content) => content.parse().unwrap_or(0.0),
            LiquidVariableType::Number(value) => *value,
            LiquidVariableType::Boolean(value) => {
                if *value {
                    1.0
                } else {
                    0.0
                }
            }
            LiquidVariableType::Array(_) => panic!("Cannot convert array to number"),
            LiquidVariableType::Object(_) => panic!("Cannot convert object to number"),
            LiquidVariableType::Nil => 0.0,
        }
    }
    pub fn convert_to_boolean(&self) -> bool {
        match self {
            LiquidVariableType::String(content) => !content.is_empty(),
            LiquidVariableType::Number(value) => *value != 0.0,
            LiquidVariableType::Boolean(value) => *value,
            LiquidVariableType::Array(_) => true,
            LiquidVariableType::Object(_) => true,
            LiquidVariableType::Nil => false,
        }
    }
}

// impl LiquidDataType {
//     pub fn convert_to_string(&self) -> String {
//         match self {
//             LiquidDataType::Liquid(variable) => variable.convert_to_string(),
//             LiquidDataType::Variable(variable) => variable.convert_to_string(),
//             LiquidDataType::String(content) => content.clone(),
//             LiquidDataType::Filter => panic!("Cannot convert filter to string"),
//             LiquidDataType::Coma => panic!("Cannot convert coma to string"),
//             LiquidDataType::Period => panic!("Cannot convert period to string"),
//             LiquidDataType::Equal => panic!("Cannot convert equal to string"),
//         }
//     }
// }
