use crate::{CLOSE_BRACKET_SYMBOL, DB_QUOTE_SYMBOL, DOT_SYMBOL, OPEN_BRACKET_SYMBOL, QUOTE_SYMBOL};

use super::types::{FastVarFinder, Instruction};
// Path is separated by dots
pub fn create_fast_var(path: &str) -> Vec<FastVarFinder> {
    let mut result: Vec<FastVarFinder> = Vec::new();

    let mut is_in_bracket = false;
    let mut is_in_quote = false;
    let mut quote_symbol: char = ' ';
    // TODO: support escape

    // TODO: change VarAsKey to ...

    let mut current = String::new();
    for c in path.chars() {
        if is_in_quote {
            if c == quote_symbol {
                is_in_quote = false;
                continue;
            }

            current.push(c);
        } else if !is_in_bracket && (c == DB_QUOTE_SYMBOL || c == QUOTE_SYMBOL) {
            panic!("Liquid error: String can't be used as a key");
        } else if is_in_bracket && (c == DB_QUOTE_SYMBOL || c == QUOTE_SYMBOL) {
            is_in_quote = true;
            quote_symbol = c;
        } else if c == OPEN_BRACKET_SYMBOL {
            is_in_bracket = true;
            result.push(FastVarFinder::Key(current));
            current = String::new();
        } else if c == CLOSE_BRACKET_SYMBOL {
            is_in_bracket = true;
            match current.parse::<u64>() {
                Ok(index) => result.push(FastVarFinder::Index(index)),
                Err(_) => result.push(FastVarFinder::VarAsKey(create_fast_var(&current))),
            }
            current = String::new();
        } else if c == DOT_SYMBOL {
            result.push(FastVarFinder::Key(current));
            current = String::new();
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        result.push(FastVarFinder::Key(current));
    }

    result
}

// Pass instructions in a function to split variables into a fast find map
// TODO: do it
pub fn fast_var_process(instructions: &mut [Instruction]) {
    for instruction in instructions.iter_mut() {
        match &mut instruction.op_type {
            crate::jit::types::InstructionType::Raw(_) => todo!(),
            crate::jit::types::InstructionType::DataManipulation(_, data, _) => {
                if let crate::jit::types::VarOrRaw::Var(name) = data {
                    *data = crate::jit::types::VarOrRaw::FastVar(create_fast_var(name));
                }
            }
        }
    }
}
