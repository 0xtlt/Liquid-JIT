use super::types::LiquidDataType;

pub fn group_by_filter(data_types: &Vec<LiquidDataType>) -> Vec<Vec<LiquidDataType>> {
    let mut groups: Vec<Vec<LiquidDataType>> = vec![vec![]];

    for data_type in data_types {
        match data_type {
            LiquidDataType::Filter => {
                groups.push(vec![]);
            }
            _ => groups.last_mut().unwrap().push(data_type.clone()),
        }
    }

    groups
}
