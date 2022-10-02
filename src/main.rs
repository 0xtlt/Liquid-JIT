use serde_json::json;

// #[derive(Debug)]
// enum LiquidMode {
//     NotLiquid,
// }

#[derive(Debug, Clone, PartialEq)]
struct Conditions {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
enum InstructionType {
    Echo,
    Error(String, u64),
    // First vec is for multiple if elseif conditions, the second is for the else case
    Conditions(Vec<Conditions>, Option<Vec<Instruction>>),
}

#[derive(Debug, Clone, PartialEq)]
enum InstructionValue {
    Float(f64),
    String(String),
    Bool(bool),
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    // 0 is echo, 1 is exit
    op_type: InstructionType,
    value: InstructionValue,
}

impl Instruction {
    // Check if the instruction actual type is the same as the expected type
    // If not, try to convert it
    pub fn convert(&mut self, new_type: InstructionValue) -> Result<(), String> {
        match new_type {
            // From x to Float
            InstructionValue::Float(_) => match self.value {
                // From Float to Float
                InstructionValue::Float(_) => Ok(()),
                // From String to Float
                InstructionValue::String(ref s) => match s.parse::<f64>() {
                    Ok(f) => {
                        self.value = InstructionValue::Float(f);
                        Ok(())
                    }
                    Err(_) => Err(format!("Cannot convert {} to number", s)),
                },
                // From Bool to Float
                InstructionValue::Bool(b) => {
                    self.value = InstructionValue::Float(if b { 1.0 } else { 0.0 });
                    Ok(())
                }
                // From Undefined to Float
                InstructionValue::Undefined => {
                    self.value = InstructionValue::Float(0.0);
                    Ok(())
                }
            },
            // From x to String
            InstructionValue::String(_) => match self.value {
                // From Float to String
                InstructionValue::Float(f) => {
                    self.value = InstructionValue::String(f.to_string());
                    Ok(())
                }
                // From String to String
                InstructionValue::String(_) => Ok(()),
                // From Bool to String
                InstructionValue::Bool(b) => {
                    self.value = InstructionValue::String(if b {
                        "true".to_owned()
                    } else {
                        "false".to_owned()
                    });
                    Ok(())
                }
                // From Undefined to String
                InstructionValue::Undefined => {
                    self.value = InstructionValue::String("".to_owned());
                    Ok(())
                }
            },
            // From x to Bool
            InstructionValue::Bool(_) => match self.value {
                // From Float to Bool
                InstructionValue::Float(f) => {
                    self.value = InstructionValue::Bool(f != 0.0);
                    Ok(())
                }
                // From String to Bool
                InstructionValue::String(ref s) => {
                    if s == "false" {
                        self.value = InstructionValue::Bool(false);
                        Ok(())
                    } else if s == "true" {
                        self.value = InstructionValue::Bool(true);
                        Ok(())
                    } else {
                        self.value = InstructionValue::Bool(!s.trim().is_empty());
                        Ok(())
                    }
                }
                // From Bool to Bool
                InstructionValue::Bool(_) => Ok(()),
                // From Undefined to Bool
                InstructionValue::Undefined => {
                    self.value = InstructionValue::Bool(false);
                    Ok(())
                }
            },
            InstructionValue::Undefined => Err("Cannot convert to undefined".to_owned()),
        }
    }
    pub fn add_char(&mut self, value: char) -> Result<(), String> {
        self.convert(InstructionValue::String(String::new()))?;

        match self.value {
            InstructionValue::String(ref mut s) => {
                s.push(value);
                Ok(())
            }
            _ => Err("Cannot add string to non-string".to_owned()),
        }
    }
}

#[derive(Debug)]
struct Instructions {
    pub instructions: Vec<Instruction>,
}
impl Instructions {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: &mut Instruction) {
        if instruction.op_type == InstructionType::Echo
            && instruction.value == InstructionValue::Undefined
        {
            // Do nothing
        } else {
            self.instructions.push(instruction.clone());
        }

        // Reset the instruction
        instruction.op_type = InstructionType::Echo;
        instruction.value = InstructionValue::Undefined;
    }

    pub fn liquid_error_handler<AnyType>(
        &mut self,
        result: Result<AnyType, String>,
        line_number: &u64,
    ) {
        match result {
            Ok(_) => (),
            Err(e) => {
                self.instructions.push(Instruction {
                    op_type: InstructionType::Error(e, *line_number),
                    value: InstructionValue::Undefined,
                });
            }
        }
    }
}

// Keywords
const FILTER_SYMBOLE: char = '|';
const QUOTE_SYMBOLE: char = '\'';
const DB_QUOTE_SYMBOLE: char = '"';
const ASSIGN_SYMBOLE: char = '=';
const ASSIGN_KEYWORD: &str = "assign";

#[derive(Debug, Clone, PartialEq)]
enum LiquidDataType {
    Liquid(String),
    Quote(String),
}

fn split_by_spaces(str: &str) -> Vec<LiquidDataType> {
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
                    result.push(LiquidDataType::Quote(current));
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
            result.push(LiquidDataType::Quote(current));
        } else {
            result.push(LiquidDataType::Liquid(current));
        }
    }

    result
}

fn compute_liquid_instructions(
    instructions: &mut Instructions,
    liquid_str: &str,
    echo_mode: &bool,
) {
    // Lines are important in liquid
    let lines = liquid_str.lines();
    for line in lines {
        let line = line.trim();

        // First pass to split by spaces (but not in quotes)
        let line_parts = split_by_spaces(line);
        println!("Line: {:?}", line_parts);
    }

    todo!("Compute liquid instructions");
}

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
        op_type: InstructionType::Echo,
        value: InstructionValue::Undefined,
    };

    let mut last_liquid_string = String::new();

    // TODO: handle errors properly to show line numbers and stuff
    let mut line_number = 1;
    for (index, letter) in file_contents.chars().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        // Get next char
        let next_char = file_contents.chars().nth(index + 1);
        let next_char_2 = file_contents.chars().nth(index + 2);

        if !is_liquid_mode {
            if !is_next_char_escaped && letter == '\\' && next_char == Some('n') {
                line_number += 1;
            } else if letter == '{' && next_char == Some('{') {
                is_liquid_mode = true;
                is_liquid_echo_mode = true;
                skip_count = 1;

                instructions.add_instruction(&mut next_instruction);
                continue;
            }

            next_instruction.op_type = InstructionType::Echo;
            instructions.liquid_error_handler(next_instruction.add_char(letter), &line_number);
            continue;
        } else if is_liquid_mode {
            if is_liquid_string_mode {
                if letter == liquid_string_mode_char {
                    is_liquid_string_mode = false;
                }
                last_liquid_string.push(letter);
            } else if !is_liquid_string_mode {
                // End of print liquid mode
                if letter == '}' && next_char == Some('}') {
                    compute_liquid_instructions(
                        &mut instructions,
                        &last_liquid_string,
                        &is_liquid_echo_mode,
                    );
                    last_liquid_string = String::new();
                    is_liquid_echo_mode = false;
                    skip_count = 1;
                    // instructions.add_instruction(&mut next_instruction);
                } else if letter == QUOTE_SYMBOLE || letter == DB_QUOTE_SYMBOLE {
                    match letter {
                        QUOTE_SYMBOLE => liquid_string_mode_char = QUOTE_SYMBOLE,
                        DB_QUOTE_SYMBOLE => liquid_string_mode_char = DB_QUOTE_SYMBOLE,
                        _ => panic!("Invalid string mode char"),
                    }
                    last_liquid_string.push(letter);
                } else {
                    last_liquid_string.push(letter);
                }
            }
        }
    }
    println!("{:?}", instructions);
    // println!("next_instruction:{:?}", next_instruction);
}
