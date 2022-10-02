use super::data_manipulation::{DataManipulation, DataManipulationMode};

#[derive(Debug, Clone, PartialEq)]
pub struct Conditions {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarOrRaw {
    Var(String),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Raw(String),
    // Error(String, u64),
    // First vec is for multiple if elseif conditions, the second is for the else case
    // Conditions(Vec<Conditions>, Option<Vec<Instruction>>),
    DataManipulation(DataManipulationMode, VarOrRaw, Vec<DataManipulation>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    // 0 is echo, 1 is exit
    pub op_type: InstructionType,
}

#[derive(Debug)]
pub struct Instructions {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiquidDataType {
    Liquid(String),
    Variable(String),
    String(String),
    // Number(f64),
    // Boolean(bool),
    // Nil,
    Filter,
    Coma,
    Period,
    Equal,
}
