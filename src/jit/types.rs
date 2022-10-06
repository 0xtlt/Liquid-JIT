use super::data_manipulation::{DataManipulation, DataManipulationMode};

#[derive(Debug, Clone, PartialEq)]
pub struct Conditions {
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastVarFinder {
    // hello.world or hello["world"] or hello['world']
    Key(String),
    // hello[hello.world]
    VarAsKey(Vec<FastVarFinder>),
    // hello[0]
    Index(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarOrRaw {
    Var(String),
    FastVar(Vec<FastVarFinder>),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    FastVar(Vec<FastVarFinder>),
    Assign,
    // Number(f64),
    // Boolean(bool),
    // Nil,
    Filter,
    Coma,
    Period,
    Equal,
}
