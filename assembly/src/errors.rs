use super::{String, ToString, Token};
use core::fmt;

// ASSEMBLY ERROR
// ================================================================================================

#[derive(Clone, Eq, PartialEq)]
pub struct AssemblyError {
    message: String,
    step: usize,
    op: String,
}

impl AssemblyError {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    pub fn empty_source() -> Self {
        AssemblyError {
            message: "source code cannot be an empty string".to_string(),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn unexpected_eof(step: usize) -> Self {
        AssemblyError {
            message: "unexpected EOF".to_string(),
            step,
            op: "".to_string(),
        }
    }

    pub fn unexpected_token(token: &Token, expected: &str) -> Self {
        AssemblyError {
            message: format!(
                "unexpected token: expected '{}' but was '{}'",
                expected, token
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn empty_block(token: &Token) -> Self {
        AssemblyError {
            message: "a code block must contain at least one instruction".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_ast_body() -> Self {
        AssemblyError {
            message: "ast body contains no instructions".to_string(),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn invalid_op(token: &Token) -> Self {
        AssemblyError {
            message: format!("instruction '{token}' is invalid"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_op_with_reason(token: &Token, reason: &str) -> Self {
        AssemblyError {
            message: format!("instruction '{token}' is invalid: {reason}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn missing_param(token: &Token) -> Self {
        AssemblyError {
            message: format!("malformed instruction '{token}': missing required parameter"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn extra_param(token: &Token) -> Self {
        AssemblyError {
            message: format!("malformed instruction '{token}': too many parameters provided"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_instruction(instruction: &str) -> Self {
        AssemblyError {
            message: format!("invalid instruction `{}` is invalid", instruction,),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn invalid_instruction_with_reason(instruction: &str, reason: &str) -> Self {
        AssemblyError {
            message: format!(
                "invalid instruction `{}` is invalid: {}",
                instruction, reason
            ),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn invalid_param(token: &Token, part_idx: usize) -> Self {
        AssemblyError {
            message: format!(
                "malformed instruction `{token}`: parameter '{}' is invalid",
                token.parts()[part_idx]
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_param_with_reason(token: &Token, part_idx: usize, reason: &str) -> Self {
        AssemblyError {
            message: format!(
                "malformed instruction '{token}', parameter {} is invalid: {reason}",
                token.parts()[part_idx],
            ),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn dangling_else(token: &Token) -> Self {
        AssemblyError {
            message: "else without matching if".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_if(token: &Token) -> Self {
        AssemblyError {
            message: "if without matching else/end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_while(token: &Token) -> Self {
        AssemblyError {
            message: "while without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_repeat(token: &Token) -> Self {
        AssemblyError {
            message: "repeat without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_else(token: &Token) -> Self {
        AssemblyError {
            message: "else without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_comment(step: usize) -> Self {
        AssemblyError {
            message: "# comment delimiter without matching #".to_string(),
            step,
            op: "".to_string(),
        }
    }

    // PROGRAM
    // --------------------------------------------------------------------------------------------

    pub fn unmatched_begin(token: &Token) -> Self {
        AssemblyError {
            message: "begin without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn dangling_ops_after_program(token: &Token) -> Self {
        AssemblyError {
            message: "dangling instructions after program end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // PROCEDURES
    // --------------------------------------------------------------------------------------------

    pub fn duplicate_proc_label(token: &Token, label: &str) -> Self {
        AssemblyError {
            message: format!("duplicate procedure label: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_label(token: &Token, label: &str) -> Self {
        AssemblyError {
            message: format!("invalid procedure label: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_proc_locals(token: &Token, locals: &str) -> Self {
        AssemblyError {
            message: format!("invalid procedure locals: {locals}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn unmatched_proc(token: &Token) -> Self {
        AssemblyError {
            message: "proc without matching end".to_string(),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn undefined_proc(token: &Token, label: &str) -> Self {
        AssemblyError {
            message: format!("undefined proc: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn undefined_imported_proc(procedure_id: String) -> Self {
        AssemblyError {
            message: format!("undefined imported procedure: {:?}", procedure_id),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn undefined_local_proc(index: u16) -> Self {
        AssemblyError {
            message: format!("undefined local procedure: {index}"),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn undefined_kernel_proc(proc_hash: &[u8; 24]) -> Self {
        AssemblyError {
            message: format!("undefined kernel procedure hash: {:?}", proc_hash),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn proc_export_not_allowed(token: &Token, label: &str) -> Self {
        AssemblyError {
            message: format!("exported procedures not allowed in this context: {label}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn proc_not_in_kernel(token: &Token, label: &str) -> Self {
        AssemblyError {
            message: format!("procedure '{label}' is not a part of the kernel"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn syscall_in_kernel() -> Self {
        AssemblyError {
            message: "syscall inside kernel".to_string(),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn call_in_kernel() -> Self {
        AssemblyError {
            message: "call inside kernel".to_string(),
            step: 0,
            op: "".to_string(),
        }
    }

    // IMPORTS AND MODULES
    // --------------------------------------------------------------------------------------------

    pub fn missing_import_source(module_path: &str) -> Self {
        AssemblyError {
            message: format!("module source not found: {}", module_path),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn dangling_ops_after_module(token: &Token, module_path: &str) -> Self {
        AssemblyError {
            message: format!("dangling instructions after module end at {module_path}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn circular_module_dependency(module_chain: &[String]) -> Self {
        AssemblyError {
            message: format!("circular module dependency in the following chain: {module_chain:?}"),
            step: 0,
            op: "".to_string(),
        }
    }

    pub fn duplicate_module_import(token: &Token, module: &str) -> Self {
        AssemblyError {
            message: format!("duplicate module import found: {module}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    pub fn invalid_module_path(token: &Token, module_path: &str) -> Self {
        AssemblyError {
            message: format!("invalid module import path: {module_path}"),
            step: token.pos(),
            op: token.to_string(),
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn operation(&self) -> &String {
        &self.op
    }

    pub fn step(&self) -> usize {
        self.step
    }
}

#[derive(Debug)]
pub enum SerializationError {
    InvalidBoolValue,
    StringTooLong,
    EndOfReader,
    InvalidOpCode,
    InvalidFieldElement,
}

// COMMON TRAIT IMPLEMENTATIONS
// ================================================================================================

impl fmt::Debug for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "assembly error at {}: {}", self.step, self.message)
    }
}
