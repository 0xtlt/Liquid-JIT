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
