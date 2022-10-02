use serde_json::Value;

use crate::jit::types::Instructions;

pub fn run(instructions: &Instructions, injected_variables: &Value) -> String {
    let mut result: String = String::new();

    result
}
