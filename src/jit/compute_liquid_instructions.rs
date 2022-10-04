use crate::{ASSIGN_SYMBOL, COMA_SYMBOL, FILTER_SYMBOL, PERIOD_SYMBOL};

use super::{
    apply_variable_detection::apply_variable_detection,
    data_manipulation::DataManipulationMode,
    generate_data_manipulations::generate_data_manipulations,
    group_by_filter::group_by_filter,
    keys_detection::keys_detection,
    split_by_spaces::split_by_spaces,
    types::{Instruction, InstructionType, Instructions, LiquidDataType, VarOrRaw},
};

pub fn compute_liquid_instructions(
    instructions: &mut Instructions,
    liquid_str: &str,
    echo_mode: bool,
) {
    // Lines are important in liquid
    let mut liquid_instructions: Vec<Vec<LiquidDataType>> = vec![];

    let lines = liquid_str.lines();
    for line in lines {
        let line = line.trim();

        // First pass to split by spaces (but not in quotes)
        let line_parts = split_by_spaces(line);

        let result = apply_variable_detection(&keys_detection(
            &keys_detection(
                &keys_detection(
                    &keys_detection(&line_parts, &FILTER_SYMBOL, LiquidDataType::Filter),
                    &COMA_SYMBOL,
                    LiquidDataType::Coma,
                ),
                &PERIOD_SYMBOL,
                LiquidDataType::Period,
            ),
            &ASSIGN_SYMBOL,
            LiquidDataType::Equal,
        ));

        if result.first() != Some(&LiquidDataType::Filter) {
            liquid_instructions.push(vec![]);
        }

        liquid_instructions.last_mut().unwrap().extend(result);
    }

    // Translate liquid instructions to instructions

    for liquid_instruction in liquid_instructions {
        let filter_groups = group_by_filter(&liquid_instruction);
        // let instruction_0 = liquid_instruction.first().expect("No instruction found");
        // let instruction_1 = liquid_instruction.first();

        let first_instruction = filter_groups.first().expect("No instruction found");

        let data = {
            if echo_mode {
                // DataManipulationMode::Echo
                first_instruction.get(0)
            } else {
                // DataManipulationMode::Assign
                first_instruction.get(1)
            }
        }
        .expect("No data found");

        let instruction: Instruction = Instruction {
            op_type: InstructionType::DataManipulation(
                {
                    if echo_mode {
                        DataManipulationMode::Echo
                    } else {
                        DataManipulationMode::Assign
                    }
                },
                match data {
                    LiquidDataType::Variable(name) => VarOrRaw::Var(name.to_owned()),
                    LiquidDataType::String(content) => VarOrRaw::Raw(content.to_owned()),
                    _ => panic!("Not supported case"),
                },
                generate_data_manipulations(&filter_groups),
                // vec![DataManipulation {
                //     args: {
                //         if function == DataManipulationFunction::Assign {
                //             vec![match first_instruction.get(3).expect("No args found") {
                //                 LiquidDataType::Variable(name) => {
                //                     DataManipulationArg::Variable(name.to_owned())
                //                 }
                //                 LiquidDataType::Quote(content) => {
                //                     DataManipulationArg::String(content.to_owned())
                //                 }
                //                 _ => panic!("Not supported case"),
                //             }]
                //         } else {
                //             vec![]
                //         }
                //     },
                //     function,
                //     arg_map: HashMap::new(),
                // }],
            ),
        };

        // instructions.push(DataManipulation {
        //     data: todo!(),
        //     function: DataManipulationFunction::Replace,
        //     args: todo!(),
        //     arg_map: todo!(),
        // });

        instructions.instructions.push(instruction);
    }
}
