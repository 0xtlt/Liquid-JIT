use crate::jit::{data_manipulation::DataManipulation, types::LiquidDataType};

use super::{
    fast_var::{get_fast_var, get_fast_var_not_mutable},
    types::LiquidVariableType,
};

fn get_arg_string(
    args: &Vec<LiquidDataType>,
    variables: &LiquidVariableType,
    index: usize,
) -> String {
    if let Some(value) = args.get(index) {
        match value {
            crate::jit::types::LiquidDataType::FastVar(fast_var_finder) => {
                match get_fast_var_not_mutable(fast_var_finder, variables) {
                    Some(variable) => variable.convert_to_string(),
                    None => "".to_string(),
                }
            }
            crate::jit::types::LiquidDataType::String(content) => content.to_string(),
            _ => panic!("Invalid argument type"),
        }
    } else {
        panic!("Missing argument 1")
    }
}

// TODO: Replace panic by liquid error
pub fn run_manipulations(
    variable: &mut LiquidVariableType,
    variables: &mut LiquidVariableType,
    manipulations: &[DataManipulation],
) {
    for manipulation in manipulations.iter() {
        match manipulation.function {
            crate::jit::data_manipulation::DataManipulationFunction::Replace => {
                let arg_1 = get_arg_string(&manipulation.args, &variables, 0);
                let arg_2 = get_arg_string(&manipulation.args, &variables, 1);

                *variable = LiquidVariableType::String(
                    variable.convert_to_string().replace(&arg_1, &arg_2),
                );
            }
            _ => (),
        }
    }
}
