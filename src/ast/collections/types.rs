pub use crate::ast::types::*;

pub enum Type {
    Primitive(Primitive),
    Combination(Combination),
}