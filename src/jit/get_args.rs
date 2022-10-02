use super::{data_manipulation::DataManipulationArgsType, types::LiquidDataType};

pub fn get_args(datas: &[LiquidDataType]) -> Vec<DataManipulationArgsType> {
    let mut args: Vec<DataManipulationArgsType> = vec![];

    let mut is_period_passed = false;
    let mut skip_count: u8 = 0;

    for (index, data) in datas.iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if !is_period_passed {
            if data == &LiquidDataType::Period {
                is_period_passed = true;
            }

            continue;
        }

        let next_type = datas.get(index + 1);

        // ... key: value
        if next_type == Some(&LiquidDataType::Period) {
            // TODO: replace the expect by a liquid error
            let next_value = datas.get(index + 2).expect("No value after period");

            skip_count = 2;

            if let LiquidDataType::String(key) = data {
                args.push(DataManipulationArgsType::ArgMap(
                    key.to_string(),
                    next_value.clone(),
                ));
            }

            continue;
        } else {
            // ... value, value, value
            args.push(DataManipulationArgsType::Arg(data.clone()));
        }
    }

    args
}
