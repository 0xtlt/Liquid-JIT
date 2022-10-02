use crate::{DB_QUOTE_SYMBOLE, QUOTE_SYMBOLE};

use super::types::LiquidDataType;

pub fn split_by_spaces(str: &str) -> Vec<LiquidDataType> {
    let mut result: Vec<LiquidDataType> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_type = ' ';
    let mut in_escaped = false;

    for c in str.chars() {
        if in_escaped {
            current.push(c);
            in_escaped = false;
        } else if c == '\\' {
            in_escaped = true;
        } else if in_quotes {
            if c == quote_type {
                if in_quotes {
                    result.push(LiquidDataType::String(current));
                    current = String::new();
                }

                in_quotes = !in_quotes;
            } else {
                current.push(c);
            }
        } else if c == DB_QUOTE_SYMBOLE {
            quote_type = DB_QUOTE_SYMBOLE;
            in_quotes = true;
        } else if c == QUOTE_SYMBOLE {
            quote_type = QUOTE_SYMBOLE;
            in_quotes = true;
        } else if c == ' ' {
            if !current.is_empty() {
                result.push(LiquidDataType::Liquid(current));
                current = String::new();
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        if in_quotes {
            result.push(LiquidDataType::String(current));
        } else {
            result.push(LiquidDataType::Liquid(current));
        }
    }

    result
}
