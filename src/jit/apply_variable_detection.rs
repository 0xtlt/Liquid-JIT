use crate::RESERVED_KEYWORDS;

use super::types::LiquidDataType;

pub fn apply_variable_detection(datas: &[LiquidDataType]) -> Vec<LiquidDataType> {
    let mut datas_with_filter: Vec<LiquidDataType> = Vec::new();
    for data in datas {
        match data {
            LiquidDataType::Liquid(content) => {
                if RESERVED_KEYWORDS.contains(&content.as_str()) {
                    datas_with_filter.push(LiquidDataType::Liquid(content.to_owned()));
                } else {
                    datas_with_filter.push(LiquidDataType::Variable(content.to_owned()));
                }
            }
            _ => {
                datas_with_filter.push(data.clone());
            }
        }
    }

    datas_with_filter
}
