use super::{
    data_manipulation::{DataManipulation, DataManipulationArgsType},
    get_args::get_args,
    get_first_liquid::get_first_liquid,
    types::LiquidDataType,
};

pub fn generate_data_manipulations(filter_groups: &[Vec<LiquidDataType>]) -> Vec<DataManipulation> {
    let mut manipulations: Vec<DataManipulation> = vec![];

    for (index, filter_group) in filter_groups.iter().enumerate() {
        if index == 0 {
            // Skip the first part because it's the variable or primary data, not manipulations
            continue;
        }

        let function = get_first_liquid(filter_group).expect("No liquid in filter group");
        let all_args = get_args(filter_group);
        let manipulation: DataManipulation = DataManipulation {
            function,
            args: all_args
                .iter()
                .filter(|arg| matches!(arg, DataManipulationArgsType::Arg(_)))
                .map(|arg| match arg {
                    DataManipulationArgsType::Arg(arg) => arg.clone(),
                    _ => panic!("Should not happen"),
                })
                .collect(),
            arg_map: all_args
                .iter()
                .filter(|arg| matches!(arg, DataManipulationArgsType::ArgMap(_, _)))
                .map(|arg| match arg {
                    DataManipulationArgsType::ArgMap(key, value) => (key.clone(), value.clone()),
                    _ => panic!("Should not happen"),
                })
                .collect(),
        };

        manipulations.push(manipulation);
    }

    manipulations
}
