use std::collections::HashMap;

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
enum VarOrRaw {
    Var(String),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq)]
enum DataManipulationFunction {
    Replace,
    Assign,
    Echo,
    NotDefined,
    Plus,
    Minus,
    T,
    Upcase,
}

#[derive(Debug, Clone, PartialEq)]
enum DataManipulationMode {
    Assign,
    Echo,
}

#[derive(Debug, Clone, PartialEq)]
enum DataManipulationArgsType {
    Arg(LiquidDataType),
    ArgMap(String, LiquidDataType),
}

#[derive(Debug, Clone, PartialEq)]
struct DataManipulation {
    function: DataManipulationFunction,
    args: Vec<LiquidDataType>,
    arg_map: HashMap<String, LiquidDataType>,
}

#[derive(Debug, Clone, PartialEq)]
enum InstructionType {
    Raw(String),
    Error(String, u64),
    // First vec is for multiple if elseif conditions, the second is for the else case
    Conditions(Vec<Conditions>, Option<Vec<Instruction>>),
    DataManipulation(DataManipulationMode, VarOrRaw, Vec<DataManipulation>),
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
}

impl Instruction {
    // Check if the instruction actual type is the same as the expected type
    // If not, try to convert it
    /*  pub fn convert(&mut self, new_type: InstructionValue) -> Result<(), String> {
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
    */
    // pub fn add_char(&mut self, value: char) -> Result<(), String> {
    //     self.convert(InstructionValue::String(String::new()))?;

    //     match self.value {
    //         InstructionValue::String(ref mut s) => {
    //             s.push(value);
    //             Ok(())
    //         }
    //         _ => Err("Cannot add string to non-string".to_owned()),
    //     }
    // }
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
        if instruction.op_type == InstructionType::Raw("".to_owned()) {
            // Do nothing
        } else {
            self.instructions.push(instruction.clone());
        }

        // Reset the instruction
        instruction.op_type = InstructionType::Raw("".to_owned());
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
                });
            }
        }
    }
}

// Keywords
const FILTER_SYMBOLE: char = '|';
const COMA_SYMBOLE: char = ',';
const PERIOD_SYMBOLE: char = ':';
const QUOTE_SYMBOLE: char = '\'';
const DB_QUOTE_SYMBOLE: char = '"';
const ASSIGN_SYMBOLE: char = '=';
const ASSIGN_KEYWORD: &str = "assign";

const RESERVED_KEYWORDS: [&str; 9] = [
    "replace", "assign", "if", "else", "endif", "plus", "minus", "t", "upcase",
];

const KEYWORDS_MAP: [DataManipulationFunction; 9] = [
    DataManipulationFunction::Replace,
    DataManipulationFunction::Assign,
    DataManipulationFunction::NotDefined,
    DataManipulationFunction::NotDefined,
    DataManipulationFunction::NotDefined,
    DataManipulationFunction::Plus,
    DataManipulationFunction::Minus,
    DataManipulationFunction::T,
    DataManipulationFunction::Upcase,
];

#[derive(Debug, Clone, PartialEq)]
enum LiquidDataType {
    Liquid(String),
    Variable(String),
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Filter,
    Coma,
    Period,
    Equal,
}

fn group_by_filter(data_types: &Vec<LiquidDataType>) -> Vec<Vec<LiquidDataType>> {
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

fn get_first_liquid(datas: &Vec<LiquidDataType>) -> Option<DataManipulationFunction> {
    for data in datas {
        if let LiquidDataType::Liquid(content) = data {
            let position = RESERVED_KEYWORDS
                .iter()
                .position(|&r| r == content)
                .unwrap();

            return Some(KEYWORDS_MAP.get(position).unwrap().clone());
        }
    }

    None
}

fn get_args(datas: &[LiquidDataType]) -> Vec<DataManipulationArgsType> {
    let mut args: Vec<DataManipulationArgsType> = vec![];

    let mut is_period_passed = false;
    let mut skip_count: u8 = 0;

    for (index, data) in datas.iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if !is_period_passed {
            if data == &LiquidDataType::Period {
                is_period_passed = true;
            }

            continue;
        }

        let next_type = datas.get(index + 1);

        // ... key: value
        if next_type == Some(&LiquidDataType::Period) {
            // TODO: replace the expect by a liquid error
            let next_value = datas.get(index + 2).expect("No value after period");

            skip_count = 2;

            if let LiquidDataType::String(key) = data {
                args.push(DataManipulationArgsType::ArgMap(
                    key.to_string(),
                    next_value.clone(),
                ));
            }

            continue;
        } else {
            // ... value, value, value
            args.push(DataManipulationArgsType::Arg(data.clone()));
        }
    }

    args
}

fn generate_data_manipulations(filter_groups: &Vec<Vec<LiquidDataType>>) -> Vec<DataManipulation> {
    let mut manipulations: Vec<DataManipulation> = vec![];

    for (index, filter_group) in filter_groups.iter().enumerate() {
        if index == 0 {
            // Skip the first part because it's the variable or primary data, not manipulations
            continue;
        }

        let function = get_first_liquid(filter_group).expect("No liquid in filter group");
        let all_args = get_args(filter_group);
        let manipulation: DataManipulation = DataManipulation {
            function,
            args: all_args
                .iter()
                .filter(|arg| matches!(arg, DataManipulationArgsType::Arg(_)))
                .map(|arg| match arg {
                    DataManipulationArgsType::Arg(arg) => arg.clone(),
                    _ => panic!("Should not happen"),
                })
                .collect(),
            arg_map: all_args
                .iter()
                .filter(|arg| matches!(arg, DataManipulationArgsType::ArgMap(_, _)))
                .map(|arg| match arg {
                    DataManipulationArgsType::ArgMap(key, value) => (key.clone(), value.clone()),
                    _ => panic!("Should not happen"),
                })
                .collect(),
        };

        manipulations.push(manipulation);
    }

    manipulations
}

fn keys_detection(
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

fn apply_variable_detection(datas: &[LiquidDataType]) -> Vec<LiquidDataType> {
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

fn compute_liquid_instructions(instructions: &mut Instructions, liquid_str: &str, echo_mode: bool) {
    // Lines are important in liquid
    let mut liquid_instructions: Vec<Vec<LiquidDataType>> = vec![];

    let lines = liquid_str.lines();
    for line in lines {
        let line = line.trim();

        // First pass to split by spaces (but not in quotes)
        let line_parts = split_by_spaces(line);

        let result = apply_variable_detection(&keys_detection(
            &keys_detection(
                &keys_detection(
                    &keys_detection(&line_parts, &FILTER_SYMBOLE, LiquidDataType::Filter),
                    &COMA_SYMBOLE,
                    LiquidDataType::Coma,
                ),
                &PERIOD_SYMBOLE,
                LiquidDataType::Period,
            ),
            &ASSIGN_SYMBOLE,
            LiquidDataType::Equal,
        ));

        if result.first() != Some(&LiquidDataType::Filter) {
            liquid_instructions.push(vec![]);
        }

        liquid_instructions.last_mut().unwrap().extend(result);
    }

    // Translate liquid instructions to instructions

    for liquid_instruction in liquid_instructions {
        let filter_groups = group_by_filter(&liquid_instruction);
        // let instruction_0 = liquid_instruction.first().expect("No instruction found");
        // let instruction_1 = liquid_instruction.first();

        let first_instruction = filter_groups.first().expect("No instruction found");

        let data = {
            if echo_mode {
                // DataManipulationMode::Echo
                first_instruction.get(0)
            } else {
                // DataManipulationMode::Assign
                first_instruction.get(1)
            }
        }
        .expect("No data found");

        let instruction: Instruction = Instruction {
            op_type: InstructionType::DataManipulation(
                {
                    if echo_mode {
                        DataManipulationMode::Echo
                    } else {
                        DataManipulationMode::Assign
                    }
                },
                match data {
                    LiquidDataType::Variable(name) => VarOrRaw::Var(name.to_owned()),
                    LiquidDataType::String(content) => VarOrRaw::Raw(content.to_owned()),
                    _ => panic!("Not supported case"),
                },
                generate_data_manipulations(&filter_groups),
                // vec![DataManipulation {
                //     args: {
                //         if function == DataManipulationFunction::Assign {
                //             vec![match first_instruction.get(3).expect("No args found") {
                //                 LiquidDataType::Variable(name) => {
                //                     DataManipulationArg::Variable(name.to_owned())
                //                 }
                //                 LiquidDataType::Quote(content) => {
                //                     DataManipulationArg::String(content.to_owned())
                //                 }
                //                 _ => panic!("Not supported case"),
                //             }]
                //         } else {
                //             vec![]
                //         }
                //     },
                //     function,
                //     arg_map: HashMap::new(),
                // }],
            ),
        };

        // instructions.push(DataManipulation {
        //     data: todo!(),
        //     function: DataManipulationFunction::Replace,
        //     args: todo!(),
        //     arg_map: todo!(),
        // });

        instructions.instructions.push(instruction);
    }
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
        op_type: InstructionType::Raw("".to_owned()),
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
