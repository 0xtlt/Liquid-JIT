pub mod convert;
pub mod fast_var;
pub mod json_to_variables;
pub mod manipulate;
pub mod types;

use std::collections::HashMap;

use serde_json::Value;

use crate::{
    jit::types::{FastVarFinder, Instruction, Instructions, VarOrRaw},
    runtime::manipulate::run_manipulations,
};

use self::{fast_var::get_fast_var, types::LiquidVariableType};

// Path can be : "a.b.c" or "a[0].b.c"
fn get_variable<'a>(
    variables: &'a mut LiquidVariableType,
    path: &'a str,
) -> Option<&'a mut LiquidVariableType> {
    let current = variables;
    let mut path = path.split('.');
    while let Some(part) = path.next() {
        if part.is_empty() {
            continue;
        }
        if part.contains('[') && part.ends_with(']') {
            todo!();
        }
    }
    Some(current)
}

fn exec(instruction: &mut Instruction, variables: &mut LiquidVariableType, end_str: &mut String) {
    let mut copy_variables = variables.clone();
    match &mut instruction.op_type {
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
            // let mut var_tmp: &mut VarOrRaw = &mut data;

            let mut var_tmp_local: LiquidVariableType;

            let mut var_tmp: &mut LiquidVariableType = match manipulation_mode {
                crate::jit::data_manipulation::DataManipulationMode::Assign => match data {
                    VarOrRaw::Var(_) => panic!("Normal var are not supported in runtime mode"),
                    VarOrRaw::FastVar(fast_var_finder) => {
                        match get_fast_var(fast_var_finder, variables) {
                            Some(variable) => variable,
                            None => {
                                // Only support 1 key for now
                                let key = fast_var_finder.get(0).unwrap();

                                if let FastVarFinder::Key(key) = key {
                                    if let LiquidVariableType::Object(object) = variables {
                                        object.insert(key.clone(), LiquidVariableType::Nil);
                                        object.get_mut(key).unwrap()
                                    } else {
                                        panic!("Can't insert a key in a non object");
                                    }
                                } else {
                                    panic!("Only support 1 key for now");
                                }
                            }
                        }
                    }
                    VarOrRaw::Raw(_) => panic!("Cannot assign to a raw value"),
                },
                // If it's for echo, clone the variable
                crate::jit::data_manipulation::DataManipulationMode::Echo => match data {
                    VarOrRaw::Var(_) => panic!("Normal var are not supported in runtime mode"),
                    VarOrRaw::FastVar(fast_var_finder) => {
                        match get_fast_var(fast_var_finder, variables) {
                            Some(variable) => {
                                var_tmp_local = variable.clone();
                                &mut var_tmp_local
                            }
                            None => {
                                var_tmp_local = LiquidVariableType::Nil;
                                &mut var_tmp_local
                            }
                        }
                    }
                    VarOrRaw::Raw(content) => {
                        var_tmp_local = LiquidVariableType::String(content.clone());
                        &mut var_tmp_local
                    }
                },
            };

            // Manipulate the variable
            // TODO: do it
            run_manipulations(var_tmp, &mut copy_variables, manipulations);

            // If manipulation mode is echo, push the variable to the end_str
            if let crate::jit::data_manipulation::DataManipulationMode::Echo = manipulation_mode {
                end_str.push_str(&var_tmp.convert_to_string());
            }
        }
    }
}

pub fn run(instructions: &mut Instructions, injected_variables: &Value) -> String {
    let mut result: String = String::new();

    let mut variables = LiquidVariableType::Object(HashMap::new());
    json_to_variables::json_to_variables(injected_variables, &mut variables);

    for instruction in instructions.instructions.iter_mut() {
        exec(instruction, &mut variables, &mut result);
    }

    result
}
