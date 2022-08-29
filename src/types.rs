#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Primitive(String),
    Struct { name: String, params: Vec<Box<Type>> },
    Ref(Box<Type>),
    RefMut(Box<Type>),
    Buffer(Box<Type>),
    Enum { name: String, variants: Vec<String> },
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum SelfType {
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

impl SelfType {
    pub fn is_ref(&self) -> bool {
        match self {
            SelfType::None => false,
            SelfType::Value => false,
            SelfType::Ref => true,
            SelfType::RefMut => true,
            SelfType::Mut => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            SelfType::None => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl Type {
    pub fn new_struct(struct_name: &str) -> Self {
        Type::Struct {
            name: struct_name.to_string(),
            params: vec![],
        }
    }

    pub fn new_ref(struct_name: &str) -> Self {
        Type::Ref(Box::new(Type::new_struct(struct_name)))
    }

    pub fn new_ref_mut(struct_name: &str) -> Self {
        Type::RefMut(Box::new(Type::new_struct(struct_name)))
    }

    pub fn new_u8_buffer() -> Self {
        Type::Buffer(Box::new(Type::Primitive("u8".to_string())))
    }

    pub fn is_buffer(&self) -> bool {
        match self {
            Type::Buffer(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Type::Struct{..} => true,
            _ => false,
        }
    }

    pub fn to_c_str(&self) -> String {
        match self {
            Type::Primitive(ty) => ty.clone(),
            Type::Struct{..} => "*mut ::std::os::raw::c_void".to_string(),
            Type::Ref(_) => "*mut ::std::os::raw::c_void".to_string(),
            Type::RefMut(_) => "*mut ::std::os::raw::c_void".to_string(),
            Type::Buffer(ty) => format!("*const {}", ty.to_c_str()),
            Type::Enum{..} => "usize".to_string(),
        }
    }

    pub fn to_rust_str(&self) -> String {
        match self {
            Type::Primitive(ty) => ty.clone(),
            Type::Struct { name, params } => if params.is_empty() {
                name.clone()
            } else {
                format!("{}<{}>", &name, params.iter().map(|p| p.to_rust_str())
                    .collect::<Vec<_>>().join(", "))
            },
            Type::Ref(ty) => format!("&{}", &ty.to_rust_str()),
            Type::RefMut(ty) => format!("&mut {}", &ty.to_rust_str()),
            Type::Buffer(ty) => format!("*const {}", match &**ty {
                Type::Ref(inner_ty) => format!("*const {}", inner_ty.to_rust_str()),
                Type::RefMut(inner_ty) => format!("*mut {}", inner_ty.to_rust_str()),
                Type::Buffer(_) => unimplemented!(),
                ty => ty.to_rust_str(),
            }),
            Type::Enum { name, .. } => name.clone(),
        }
    }
}
