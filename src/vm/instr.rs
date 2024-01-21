use std::rc::Rc;

use bincode::{Decode, Encode};

use crate::calx::{Calx, CalxType};

/// learning from WASM but for dynamic data
#[derive(Debug, Clone, PartialEq, PartialOrd, Decode, Encode)]
pub enum CalxInstr {
  // Param, // load variable from parameter
  LocalSet(usize),
  LocalTee(usize), // set and also load to stack
  LocalGet(usize),
  LocalNew,
  GlobalSet(usize),
  GlobalGet(usize),
  GlobalNew,
  Const(Calx),
  Dup,
  Drop,
  // number operations
  IntAdd,
  IntMul,
  IntDiv,
  IntRem,
  IntNeg,
  IntShr,
  IntShl,
  /// equal
  IntEq,
  /// not equal
  IntNe,
  /// littler than
  IntLt,
  /// littler than, or equal
  IntLe,
  /// greater than
  IntGt,
  /// greater than, or equal
  IntGe,
  Add,
  Mul,
  Div,
  Neg,
  // string operations
  // list operations
  NewList,
  ListGet,
  ListSet,
  // Link
  NewLink,
  // bool operations
  And,
  Or,
  Not,
  // control stuctures
  Br(usize),
  BrIf(usize),
  Jmp(usize),   // internal
  JmpIf(usize), // internal
  Block {
    params_types: Rc<Vec<CalxType>>,
    ret_types: Rc<Vec<CalxType>>,
    /// bool to indicate loop
    looped: bool,
    from: usize,
    to: usize,
  },
  BlockEnd(bool),
  /// pop and println current value
  Echo,
  /// TODO use function name at first, during running, index can be faster
  Call(String),
  /// for tail recursion
  ReturnCall(String),
  CallImport(String),
  Unreachable,
  Nop,
  Quit(usize), // quit and return value
  Return,
  /// TODO might also be a foreign function instead
  Assert(String),
  /// inspecting stack
  Inspect,
  /// if takes 1 value from stack, returns values as ret_types
  If {
    ret_types: Rc<Vec<CalxType>>,
    then_to: usize,
    else_to: usize,
    to: usize,
  },
  EndIf,
}

impl CalxInstr {
  /// notice that some of the instrs are special and need to handle manually
  pub fn stack_arity(&self) -> (usize, usize) {
    match self {
      CalxInstr::LocalSet(_) => (1, 0),
      CalxInstr::LocalTee(_) => (1, 1), // TODO need check
      CalxInstr::LocalGet(_) => (0, 1),
      CalxInstr::LocalNew => (0, 0),
      CalxInstr::GlobalSet(_) => (1, 0),
      CalxInstr::GlobalGet(_) => (0, 1),
      CalxInstr::GlobalNew => (0, 0),
      CalxInstr::Const(_) => (0, 1),
      CalxInstr::Dup => (1, 2),
      CalxInstr::Drop => (1, 0),
      CalxInstr::IntAdd => (2, 1),
      CalxInstr::IntMul => (2, 1),
      CalxInstr::IntDiv => (2, 1),
      CalxInstr::IntRem => (2, 1),
      CalxInstr::IntNeg => (1, 1),
      CalxInstr::IntShr => (2, 1),
      CalxInstr::IntShl => (2, 1),
      CalxInstr::IntEq => (2, 1),
      CalxInstr::IntNe => (2, 1),
      CalxInstr::IntLt => (2, 1),
      CalxInstr::IntLe => (2, 1),
      CalxInstr::IntGt => (2, 1),
      CalxInstr::IntGe => (2, 1),
      CalxInstr::Add => (2, 1),
      CalxInstr::Mul => (2, 1),
      CalxInstr::Div => (2, 1),
      CalxInstr::Neg => (1, 1),
      // string operations
      // list operations
      CalxInstr::NewList => (0, 1),
      CalxInstr::ListGet => (2, 1),
      CalxInstr::ListSet => (3, 0),
      // Link
      CalxInstr::NewLink => (0, 1),
      // bool operations
      CalxInstr::And => (2, 1),
      CalxInstr::Or => (2, 1),
      CalxInstr::Not => (1, 1),
      // control stuctures
      CalxInstr::Br(_) => (0, 0),
      CalxInstr::BrIf(_) => (1, 0),
      CalxInstr::Jmp(_) => (0, 0),
      CalxInstr::JmpIf(_) => (1, 0),
      CalxInstr::Block {
        params_types, ret_types, ..
      } => (params_types.len(), ret_types.len()),
      CalxInstr::BlockEnd(_) => (0, 0),
      CalxInstr::Echo => (1, 0),
      CalxInstr::Call(_) => (0, 0),       // TODO
      CalxInstr::ReturnCall(_) => (0, 0), // TODO
      CalxInstr::CallImport(_) => (0, 0), // import
      CalxInstr::Unreachable => (0, 0),   // TODO
      CalxInstr::Nop => (0, 0),
      CalxInstr::Quit(_) => (0, 0),
      CalxInstr::Return => (1, 0), // TODO
      CalxInstr::Assert(_) => (1, 0),
      // debug
      CalxInstr::Inspect => (0, 0),
      CalxInstr::If { ret_types, .. } => (1, ret_types.len()),
      CalxInstr::EndIf => (0, 0),
    }
  }
}

/// TODO not sure whether bincode remains compatible after new instruction added
/// use string for some semantics
pub const CALX_INSTR_EDITION: &str = "0.2";