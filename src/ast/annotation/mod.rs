use derive_more::From;

pub mod built_in;
pub mod combination;
pub mod function;
pub mod optional;
pub mod primitive;
pub mod singleton;
pub mod table;

pub use built_in::TypeBuiltIn;
pub use combination::TypeCombination;
pub use function::{TypeArgument, TypeFunction};
pub use optional::TypeOptional;
pub use primitive::TypePrimitive;
pub use singleton::TypeSingleton;
pub use table::{TypeArray, TypeTable, TypeTableEntry, TypeTableIndexer};

#[derive(Debug, Clone, PartialEq, From)]
pub enum TypeAnnotation {
    BuiltIn(TypeBuiltIn),
    Primitive(TypePrimitive),
    Function(TypeFunction),
    Table(TypeTable),
    Array(TypeArray),
    Singleton(TypeSingleton),
    Optional(TypeOptional),
    Combination(TypeCombination),
}
