#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ArgType {
    Primitive { ty: String },
    Struct { inner_ty: String },
    Ref { inner_ty: String },
    RefMut { inner_ty: String },
    Buffer,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SelfArgType {
    Value,
    Ref,
    RefMut,
    Mut,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum DerivedTrait {
    Default,
    Clone,
    PartialEq,
    Eq,
}
