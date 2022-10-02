use crate::{KEYWORDS_MAP, RESERVED_KEYWORDS};

use super::{data_manipulation::DataManipulationFunction, types::LiquidDataType};

pub fn get_first_liquid(datas: &Vec<LiquidDataType>) -> Option<DataManipulationFunction> {
    for data in datas {
        if let LiquidDataType::Liquid(content) = data {
            let position = RESERVED_KEYWORDS
                .iter()
                .position(|&r| r == content)
                .unwrap();

            return Some(KEYWORDS_MAP.get(position).unwrap().clone());
        }
    }

    None
}
