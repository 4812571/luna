use crate::ast::TypeAnnotation;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeTableIndexer {
    index: TypeAnnotation,
    value: TypeAnnotation,
}

impl TypeTableIndexer {
    /// Construct a new `TypeTableIndexer` with the given `index` and `value`.
    pub fn new(index: TypeAnnotation, value: TypeAnnotation) -> Self {
        Self { index, value }
    }

    /// Returns the index of this `TypeTableIndexer`.
    pub fn index(&self) -> &TypeAnnotation {
        &self.index
    }

    /// Returns the value of this `TypeTableIndexer`.
    pub fn value(&self) -> &TypeAnnotation {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeTableEntry {
    index: String,
    value: TypeAnnotation,
}

impl TypeTableEntry {
    /// Construct a new `TypeTableEntry` with the given `index` and `value`.
    pub fn new(index: String, value: TypeAnnotation) -> Self {
        Self { index, value }
    }

    /// Returns the index of this `TypeTableEntry`.
    pub fn index(&self) -> &str {
        &self.index
    }

    /// Returns the value of this `TypeTableEntry`.
    pub fn value(&self) -> &TypeAnnotation {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeTable {
    entries: Vec<TypeTableEntry>,
    indexer: Option<Box<TypeTableIndexer>>,
}

impl TypeTable {
    /// Construct a new `Table` with the given `entries` and `indexer`.
    pub fn new(entries: Vec<TypeTableEntry>, indexer: Option<Box<TypeTableIndexer>>) -> Self {
        Self { entries, indexer }
    }

    /// Returns the entries of this `Table`.
    pub fn entries(&self) -> &Vec<TypeTableEntry> {
        &self.entries
    }

    /// Returns the indexer of this `Table`.
    pub fn indexer(&self) -> &Option<Box<TypeTableIndexer>> {
        &self.indexer
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArray {
    item: Option<Box<TypeAnnotation>>,
}

impl TypeArray {
    pub fn new(item: Option<Box<TypeAnnotation>>) -> Self {
        Self { item }
    }

    pub fn item(&self) -> &Option<Box<TypeAnnotation>> {
        &self.item
    }
}
