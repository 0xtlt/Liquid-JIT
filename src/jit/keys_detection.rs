use super::types::LiquidDataType;

pub fn keys_detection(
    datas: &[LiquidDataType],
    symbole: &char,
    kind: LiquidDataType,
) -> Vec<LiquidDataType> {
    let mut datas_with_filter: Vec<LiquidDataType> = Vec::new();
    for data in datas {
        match data {
            LiquidDataType::Liquid(content) => {
                if content.contains(*symbole) {
                    let parts = content.split(*symbole).collect::<Vec<&str>>();

                    for (index, part) in parts.iter().enumerate() {
                        if !part.is_empty() {
                            datas_with_filter.push(LiquidDataType::Liquid(part.to_string()));
                        }

                        if index < parts.len() - 1 {
                            datas_with_filter.push(kind.clone());
                        }
                    }
                } else {
                    datas_with_filter.push(LiquidDataType::Liquid(content.to_owned()));
                }
            }
            _ => {
                datas_with_filter.push(data.clone());
            }
        }
    }

    datas_with_filter
}
