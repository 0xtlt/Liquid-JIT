use std::collections::HashMap;

use serde_json::Value;

use super::types::LiquidVariableType;

pub fn json_to_variables(json: &Value, variables: &mut LiquidVariableType) {
    match variables {
        LiquidVariableType::Array(array) => {
            let json_array = json.as_array().unwrap();
            for value in json_array {
                if value.is_f64() {
                    array.push(LiquidVariableType::Number(value.as_f64().unwrap()));
                } else if value.is_string() {
                    array.push(LiquidVariableType::String(
                        value.as_str().unwrap().to_string(),
                    ));
                } else if value.is_i64() {
                    array.push(LiquidVariableType::Number(value.as_i64().unwrap() as f64));
                } else if value.is_object() {
                    let mut object = LiquidVariableType::Object(HashMap::new());
                    json_to_variables(value, &mut object);
                    array.push(object);
                } else if value.is_array() {
                    let mut array_bis = LiquidVariableType::Array(vec![]);
                    json_to_variables(value, &mut array_bis);
                    array.push(array_bis);
                } else {
                    panic!("Not implemented {:?}", value);
                }
            }
        }
        LiquidVariableType::Object(obj) => {
            let keys = json.as_object().unwrap().keys();
            for key in keys {
                let value = json.get(key).unwrap();

                if value.is_f64() {
                    obj.insert(
                        key.to_string(),
                        LiquidVariableType::Number(value.as_f64().unwrap()),
                    );
                } else if value.is_string() {
                    obj.insert(
                        key.to_string(),
                        LiquidVariableType::String(value.as_str().unwrap().to_string()),
                    );
                } else if value.is_i64() {
                    obj.insert(
                        key.to_string(),
                        LiquidVariableType::Number(value.as_i64().unwrap() as f64),
                    );
                } else if value.is_object() {
                    let mut object = LiquidVariableType::Object(HashMap::new());
                    json_to_variables(value, &mut object);
                    obj.insert(key.to_string(), object);
                } else if value.is_array() {
                    let mut array = LiquidVariableType::Array(vec![]);
                    json_to_variables(value, &mut array);
                    obj.insert(key.to_string(), array);
                } else {
                    panic!("Not implemented {:?}", value);
                }
            }
        }
        _ => panic!("Not implemented {:?}", variables),
    }
}
