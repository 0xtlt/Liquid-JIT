pub mod json_to_variables;
pub mod types;

use std::collections::HashMap;

use serde_json::Value;

use crate::jit::types::Instructions;

use self::types::LiquidVariableType;

pub fn run(instructions: &Instructions, injected_variables: &Value) -> String {
    let mut result: String = String::new();

    let mut variables = LiquidVariableType::Object(HashMap::new());
    json_to_variables::json_to_variables(injected_variables, &mut variables);

    println!("Variables: {:?}", variables);

    result
}
