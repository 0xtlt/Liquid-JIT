use super::types::{FastVarFinder, Instruction};

// Path is separated by dots
pub fn create_fast_var(path: &str) -> Vec<FastVarFinder> {
    todo!()
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
