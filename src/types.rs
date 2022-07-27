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

impl SelfArgType {
    pub fn is_ref(&self) -> bool {
        match self {
            SelfArgType::Value => false,
            SelfArgType::Ref => true,
            SelfArgType::RefMut => true,
            SelfArgType::Mut => false,
        }
    }
}

impl ArgType {
    pub fn is_buffer(&self) -> bool {
        match self {
            ArgType::Buffer => true,
            _ => false,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            ArgType::Primitive { ty } => ty,
            ArgType::Struct{..} => "*mut ::std::os::raw::c_void",
            ArgType::Ref{..} => "*mut ::std::os::raw::c_void",
            ArgType::RefMut{..} => "*mut ::std::os::raw::c_void",
            ArgType::Buffer => "*const ::std::os::raw::c_uchar",
        }
    }
}
