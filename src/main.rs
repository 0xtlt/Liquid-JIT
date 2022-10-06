mod jit;
pub mod runtime;

use jit::{
    data_manipulation::DataManipulationFunction,
    types::{Instruction, InstructionType, Instructions},
};
use serde_json::json;

use crate::jit::{
    compute_liquid_instructions::compute_liquid_instructions,
    fast_var::{create_fast_var, fast_var_process},
};

// TODO: add number support aside of string one

// impl Instruction {
//     // Check if the instruction actual type is the same as the expected type
//     // If not, try to convert it
//     /*  pub fn convert(&mut self, new_type: InstructionValue) -> Result<(), String> {
//         match new_type {
//             // From x to Float
//             InstructionValue::Float(_) => match self.value {
//                 // From Float to Float
//                 InstructionValue::Float(_) => Ok(()),
//                 // From String to Float
//                 InstructionValue::String(ref s) => match s.parse::<f64>() {
//                     Ok(f) => {
//                         self.value = InstructionValue::Float(f);
//                         Ok(())
//                     }
//                     Err(_) => Err(format!("Cannot convert {} to number", s)),
//                 },
//                 // From Bool to Float
//                 InstructionValue::Bool(b) => {
//                     self.value = InstructionValue::Float(if b { 1.0 } else { 0.0 });
//                     Ok(())
//                 }
//                 // From Undefined to Float
//                 InstructionValue::Undefined => {
//                     self.value = InstructionValue::Float(0.0);
//                     Ok(())
//                 }
//             },
//             // From x to String
//             InstructionValue::String(_) => match self.value {
//                 // From Float to String
//                 InstructionValue::Float(f) => {
//                     self.value = InstructionValue::String(f.to_string());
//                     Ok(())
//                 }
//                 // From String to String
//                 InstructionValue::String(_) => Ok(()),
//                 // From Bool to String
//                 InstructionValue::Bool(b) => {
//                     self.value = InstructionValue::String(if b {
//                         "true".to_owned()
//                     } else {
//                         "false".to_owned()
//                     });
//                     Ok(())
//                 }
//                 // From Undefined to String
//                 InstructionValue::Undefined => {
//                     self.value = InstructionValue::String("".to_owned());
//                     Ok(())
//                 }
//             },
//             // From x to Bool
//             InstructionValue::Bool(_) => match self.value {
//                 // From Float to Bool
//                 InstructionValue::Float(f) => {
//                     self.value = InstructionValue::Bool(f != 0.0);
//                     Ok(())
//                 }
//                 // From String to Bool
//                 InstructionValue::String(ref s) => {
//                     if s == "false" {
//                         self.value = InstructionValue::Bool(false);
//                         Ok(())
//                     } else if s == "true" {
//                         self.value = InstructionValue::Bool(true);
//                         Ok(())
//                     } else {
//                         self.value = InstructionValue::Bool(!s.trim().is_empty());
//                         Ok(())
//                     }
//                 }
//                 // From Bool to Bool
//                 InstructionValue::Bool(_) => Ok(()),
//                 // From Undefined to Bool
//                 InstructionValue::Undefined => {
//                     self.value = InstructionValue::Bool(false);
//                     Ok(())
//                 }
//             },
//             InstructionValue::Undefined => Err("Cannot convert to undefined".to_owned()),
//         }
//     }
//     */
//     // pub fn add_char(&mut self, value: char) -> Result<(), String> {
//     //     self.convert(InstructionValue::String(String::new()))?;

//     //     match self.value {
//     //         InstructionValue::String(ref mut s) => {
//     //             s.push(value);
//     //             Ok(())
//     //         }
//     //         _ => Err("Cannot add string to non-string".to_owned()),
//     //     }
//     // }
// }

impl Instructions {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: &mut Instruction) {
        if instruction.op_type == InstructionType::Raw("".to_owned()) {
            // Do nothing
        } else {
            self.instructions.push(instruction.clone());
        }

        // Reset the instruction
        instruction.op_type = InstructionType::Raw("".to_owned());
    }

    // pub fn liquid_error_handler<AnyType>(
    //     &mut self,
    //     result: Result<AnyType, String>,
    //     line_number: &u64,
    // ) {
    //     match result {
    //         Ok(_) => (),
    //         Err(e) => {
    //             self.instructions.push(Instruction {
    //                 op_type: InstructionType::Error(e, *line_number),
    //             });
    //         }
    //     }
    // }
}

// Keywords
const FILTER_SYMBOL: char = '|';
const COMA_SYMBOL: char = ',';
const PERIOD_SYMBOL: char = ':';
const QUOTE_SYMBOL: char = '\'';
const DB_QUOTE_SYMBOL: char = '"';
const ASSIGN_SYMBOL: char = '=';
const DOT_SYMBOL: char = '.';
const OPEN_BRACKET_SYMBOL: char = '[';
const CLOSE_BRACKET_SYMBOL: char = ']';
// const ASSIGN_KEYWORD: &str = "assign";

const RESERVED_KEYWORDS: [&str; 10] = [
    "replace", "assign", "if", "else", "endif", "plus", "minus", "t", "upcase", "downcase",
];

const KEYWORDS_MAP: [DataManipulationFunction; 10] = [
    DataManipulationFunction::Replace,
    DataManipulationFunction::Assign,
    DataManipulationFunction::NotDefined,
    DataManipulationFunction::NotDefined,
    DataManipulationFunction::NotDefined,
    DataManipulationFunction::Plus,
    DataManipulationFunction::Minus,
    DataManipulationFunction::T,
    DataManipulationFunction::Upcase,
    DataManipulationFunction::Downcase,
];

fn main() {
    // read hello.liquid
    let file_contents = std::fs::read_to_string("hello.liquid").unwrap();

    let variables = json!({
        "product": json!({
            "title": "Liquid",
            "price": 19.95,
            "tags": json!(["liquid", "template", "engine"])
        })
    });

    let mut is_liquid_mode = false;
    let mut is_liquid_echo_mode = false;
    let mut is_liquid_string_mode = false;
    let mut liquid_string_mode_char: char = ' ';
    let mut is_next_char_escaped = false;
    let mut skip_count = 0;

    let mut instructions = Instructions::new();
    let mut next_instruction = Instruction {
        op_type: InstructionType::Raw("".to_owned()),
    };

    let mut last_liquid_string = String::new();

    // TODO: handle errors properly to show line numbers and stuff
    // let mut line_number = 1;
    for (index, letter) in file_contents.chars().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        // Get next char
        let next_char = file_contents.chars().nth(index + 1);
        // TODO: use it later for {%- -%} and {{- -}} and stuff
        // let next_char_2 = file_contents.chars().nth(index + 2);

        if !is_liquid_mode {
            if !is_next_char_escaped && letter == '\\' && next_char == Some('n') {
                // line_number += 1;
            } else if letter == '{' && next_char == Some('{') {
                is_liquid_mode = true;
                is_liquid_echo_mode = true;
                skip_count = 1;

                instructions.add_instruction(&mut next_instruction);
                continue;
            } else if letter == '{' && next_char == Some('%') {
                is_liquid_mode = true;
                is_liquid_echo_mode = false;
                skip_count = 1;

                instructions.add_instruction(&mut next_instruction);
                continue;
            }

            next_instruction.op_type = {
                match &next_instruction.op_type {
                    InstructionType::Raw(content) => {
                        InstructionType::Raw(format!("{}{}", content, letter))
                    }
                    _ => InstructionType::Raw(letter.to_string()),
                }
            };
            // instructions.liquid_error_handler(next_instruction.add_char(letter), &line_number);
            continue;
        } else if is_liquid_mode {
            if is_liquid_string_mode {
                if letter == liquid_string_mode_char {
                    is_liquid_string_mode = false;
                }
                last_liquid_string.push(letter);
            } else if !is_liquid_string_mode {
                // End of print liquid mode
                if (letter == '}' || letter == '%') && next_char == Some('}') {
                    compute_liquid_instructions(
                        &mut instructions,
                        &last_liquid_string,
                        is_liquid_echo_mode,
                    );
                    last_liquid_string = String::new();
                    is_liquid_mode = false;
                    is_liquid_echo_mode = false;
                    skip_count = 1;
                    // instructions.add_instruction(&mut next_instruction);
                } else if letter == QUOTE_SYMBOL || letter == DB_QUOTE_SYMBOL {
                    match letter {
                        QUOTE_SYMBOL => liquid_string_mode_char = QUOTE_SYMBOL,
                        DB_QUOTE_SYMBOL => liquid_string_mode_char = DB_QUOTE_SYMBOL,
                        _ => panic!("Invalid string mode char"),
                    }
                    last_liquid_string.push(letter);
                } else {
                    last_liquid_string.push(letter);
                }
            }
        }
    }

    // Append last instruction
    instructions.add_instruction(&mut next_instruction);

    // Pass instructions in a function to split variables into a fast find map
    println!("hello.world['name'][product.variants]");
    println!(
        "{:?}",
        create_fast_var("hello.world['name'][product.variants]")
    );
    println!();
    println!("product.variants[section.blocks[0].id]");
    println!(
        "{:?}",
        create_fast_var("product.variants[section.blocks[0].id]")
    );
    fast_var_process(&mut instructions.instructions);

    println!("Instructions:");
    println!("{:?}", instructions);

    println!();
    println!();
    println!();

    println!("RUN Result:");
    println!("{}", runtime::run(&mut instructions, &variables));

    println!("Variables after run:");
    println!("{:?}", variables);

    // println!("next_instruction:{:?}", next_instruction);
}
