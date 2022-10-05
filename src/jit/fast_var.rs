use crate::{CLOSE_BRACKET_SYMBOL, DB_QUOTE_SYMBOL, DOT_SYMBOL, OPEN_BRACKET_SYMBOL, QUOTE_SYMBOL};

use super::types::{FastVarFinder, Instruction};
// Path is separated by dots
pub fn create_fast_var(path: &str) -> Vec<FastVarFinder> {
    let mut result: Vec<FastVarFinder> = Vec::new();

    let mut is_in_bracket = false;
    let mut is_in_quote = false;
    let mut quote_type = QUOTE_SYMBOL;
    // TODO: support escape

    // TODO: change VarAsKey to ...

    let mut current = String::new();

    // Detect bracket encapsulation like "a[d[e]].b.c"
    let mut bracket_level = 0;

    for c in path.chars() {
        // println!("{}: {} = {:?}", c, path, current);
        if !is_in_bracket && (c == DB_QUOTE_SYMBOL || c == QUOTE_SYMBOL) {
            panic!("Liquid error: String can't be used as a key");
        } else if c == DB_QUOTE_SYMBOL || c == QUOTE_SYMBOL {
            if is_in_quote {
                if quote_type == c {
                    is_in_quote = false;
                }
            } else {
                is_in_quote = true;
                quote_type = c;
            }

            current.push(c);
        } else if c == OPEN_BRACKET_SYMBOL && !is_in_quote {
            if bracket_level == 0 {
                is_in_bracket = true;
                result.push(FastVarFinder::Key(current));
                current = String::new();
            } else {
                current.push(c);
            }
            bracket_level += 1;
        } else if c == CLOSE_BRACKET_SYMBOL && !is_in_quote {
            bracket_level -= 1;

            if bracket_level > 0 {
                current.push(c);
                continue;
            }

            is_in_bracket = false;
            match current.parse::<u64>() {
                Ok(index) => result.push(FastVarFinder::Index(index)),
                Err(_) => {
                    if (current.starts_with(DB_QUOTE_SYMBOL) && current.ends_with(DB_QUOTE_SYMBOL))
                        || (current.starts_with(QUOTE_SYMBOL) && current.ends_with(QUOTE_SYMBOL))
                    {
                        result.push(FastVarFinder::Key(
                            current[1..current.len() - 1].to_string(),
                        ));
                    } else {
                        result.push(FastVarFinder::VarAsKey(create_fast_var(&current)))
                    }
                }
            }
            current = String::new();
        } else if c == DOT_SYMBOL && !is_in_bracket {
            result.push(FastVarFinder::Key(current));
            current = String::new();
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        result.push(FastVarFinder::Key(current));
    }

    // Filter empty keys
    result.retain(|x| match x {
        FastVarFinder::Key(key) => !key.is_empty(),
        _ => true,
    });

    result
}

// Pass instructions in a function to split variables into a fast find map
// TODO: do it
pub fn fast_var_process(instructions: &mut [Instruction]) {
    for instruction in instructions.iter_mut() {
        match &mut instruction.op_type {
            crate::jit::types::InstructionType::Raw(_) => {}
            crate::jit::types::InstructionType::DataManipulation(_, data, _) => {
                if let crate::jit::types::VarOrRaw::Var(name) = data {
                    *data = crate::jit::types::VarOrRaw::FastVar(create_fast_var(name));
                }
            }
        }
    }
}
