pub mod json_to_variables;
pub mod types;

use std::collections::HashMap;

use serde_json::Value;

use crate::jit::types::{Instruction, Instructions};

use self::types::LiquidVariableType;

fn exec(instruction: &Instruction, variables: &mut LiquidVariableType, end_str: &mut String) {
    match &instruction.op_type {
        crate::jit::types::InstructionType::Raw(content) => {
            end_str.push_str(content);
        }
        crate::jit::types::InstructionType::DataManipulation(
            manipulation_mode,
            data,
            manipulations,
        ) => {}
    }
}

pub fn run(instructions: &Instructions, injected_variables: &Value) -> String {
    let mut result: String = String::new();

    let mut variables = LiquidVariableType::Object(HashMap::new());
    json_to_variables::json_to_variables(injected_variables, &mut variables);

    for instruction in instructions.instructions.iter() {
        exec(instruction, &mut variables, &mut result);
    }

    println!("Variables: {:?}", variables);

    result
}
