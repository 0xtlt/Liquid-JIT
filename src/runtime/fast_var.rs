use crate::jit::types::FastVarFinder;

use super::types::LiquidVariableType;

pub fn get_fast_var<'a>(
    path: &'a Vec<FastVarFinder>,
    variables: &'a mut LiquidVariableType,
) -> Option<&'a mut LiquidVariableType> {
    let mut var_clone = variables.clone();
    let mut current = variables;

    for part in path {
        match part {
            FastVarFinder::Key(key) => {
                if let LiquidVariableType::Object(object) = current {
                    current = object.get_mut(key)?;
                } else {
                    return None;
                }
            }
            FastVarFinder::Index(index) => {
                if let LiquidVariableType::Array(array) = current {
                    current = array.get_mut(*index as usize)?;
                } else {
                    return None;
                }
            }
            FastVarFinder::VarAsKey(fast_var) => {
                let key = get_fast_var(fast_var, &mut var_clone)?;

                if let LiquidVariableType::String(key) = key {
                    if let LiquidVariableType::Object(object) = current {
                        current = object.get_mut(key)?;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }
    }

    Some(current)
}

pub fn get_fast_var_not_mutable<'a>(
    path: &'a Vec<FastVarFinder>,
    variables: &'a LiquidVariableType,
) -> Option<&'a LiquidVariableType> {
    let mut var_clone = variables.clone();
    let mut current = variables;

    for part in path {
        match part {
            FastVarFinder::Key(key) => {
                if let LiquidVariableType::Object(object) = current {
                    current = object.get(key)?;
                } else {
                    return None;
                }
            }
            FastVarFinder::Index(index) => {
                if let LiquidVariableType::Array(array) = current {
                    current = array.get(*index as usize)?;
                } else {
                    return None;
                }
            }
            FastVarFinder::VarAsKey(fast_var) => {
                let key = get_fast_var(fast_var, &mut var_clone)?;

                if let LiquidVariableType::String(key) = key {
                    if let LiquidVariableType::Object(object) = current {
                        current = object.get(key)?;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }
    }

    Some(current)
}
