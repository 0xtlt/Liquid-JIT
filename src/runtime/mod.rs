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
        ) => {
            // Get the variable pointer in the var_tmp variabke
            // TODO: do it
            let mut var_tmp = match manipulation_mode {
                crate::jit::data_manipulation::DataManipulationMode::Assign => todo!(),
                crate::jit::data_manipulation::DataManipulationMode::Echo => todo!(),
            };

            // Manipulate the variable
            // TODO: do it

            // If manipulation mode is echo, push the variable to the end_str
            // TODO: do it
        }
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
