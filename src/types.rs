#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ArgType {
    Primitive(String),
    Struct { name: String, params: Vec<Box<ArgType>> },
    Ref(Box<ArgType>),
    RefMut(Box<ArgType>),
    Buffer,
    Enum { name: String, variants: Vec<String> },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum SelfArgType {
    None,
    Value,
    Ref,
    RefMut,
    Mut,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DerivedTrait {
    Default,
    Clone,
    PartialEq,
    Eq,
}

impl SelfArgType {
    pub fn is_ref(&self) -> bool {
        match self {
            SelfArgType::None => false,
            SelfArgType::Value => false,
            SelfArgType::Ref => true,
            SelfArgType::RefMut => true,
            SelfArgType::Mut => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            SelfArgType::None => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl ArgType {
    pub fn new_struct(struct_name: &str) -> Self {
        ArgType::Struct {
            name: struct_name.to_string(),
            params: vec![],
        }
    }

    pub fn new_ref(struct_name: &str) -> Self {
        ArgType::Ref(Box::new(ArgType::new_struct(struct_name)))
    }

    pub fn new_ref_mut(struct_name: &str) -> Self {
        ArgType::RefMut(Box::new(ArgType::new_struct(struct_name)))
    }

    pub fn is_buffer(&self) -> bool {
        match self {
            ArgType::Buffer => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            ArgType::Struct{..} => true,
            _ => false,
        }
    }

    pub fn to_c_str(&self) -> &str {
        match self {
            ArgType::Primitive(ty) => ty,
            ArgType::Struct{..} => "*mut ::std::os::raw::c_void",
            ArgType::Ref{..} => "*mut ::std::os::raw::c_void",
            ArgType::RefMut{..} => "*mut ::std::os::raw::c_void",
            ArgType::Buffer => "*const u8",
            ArgType::Enum{..} => "usize",
        }
    }

    pub fn to_rust_str(&self) -> String {
        match self {
            ArgType::Primitive(ty) => ty.clone(),
            ArgType::Struct { name, params } => if params.is_empty() {
                name.clone()
            } else {
                format!("{}<{}>", &name, params.iter().map(|p| p.to_rust_str())
                    .collect::<Vec<_>>().join(", "))
            },
            ArgType::Ref(ty) => format!("&{}", &ty.to_rust_str()),
            ArgType::RefMut(ty) => format!("&mut {}", &ty.to_rust_str()),
            ArgType::Buffer => unimplemented!(),
            ArgType::Enum { name, .. } => name.clone(),
        }
    }
}
