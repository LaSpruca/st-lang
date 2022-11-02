pub enum Identifier {
    Identifier { module: String, path: Vec<String> },
    LocalIdentifier { path: String },
    ScopeIdentifier { name: String },
    SelfIdentifier { field: String },
}

pub struct Type {
    identifier: Identifier,
    typeargs
}
