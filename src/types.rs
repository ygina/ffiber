#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Primitive(String),
    Struct { name: String, args: Vec<Box<Type>> },
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
    ValueMut,
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
            SelfType::ValueMut => false,
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
            args: vec![],
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
            Type::Struct { name, args } => if args.is_empty() {
                name.clone()
            } else {
                format!("{}<{}>", &name, args.iter().map(|p| p.to_rust_str())
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

static PRIMITIVE_TYPES: [&str; 9] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "bool",
];

impl From<syn::Receiver> for SelfType {
    fn from(item: syn::Receiver) -> Self {
        if !item.attrs.is_empty() {
            unimplemented!()
        }
        if let Some((_, ref lifetime)) = item.reference {
            if lifetime.is_some() {
                unimplemented!()
            }
        }
        match (item.reference.is_some(), item.mutability.is_some()) {
            (true, true) => Self::RefMut,
            (true, false) => Self::Ref,
            (false, true) => Self::ValueMut,
            (false, false) => Self::Value,
        }
    }
}

impl From<syn::FnArg> for SelfType {
    fn from(item: syn::FnArg) -> Self {
        match item {
            syn::FnArg::Receiver(receiver) => Self::from(receiver),
            syn::FnArg::Typed(_) => SelfType::None,
        }
    }
}

impl From<syn::Type> for Type {
    fn from(ty: syn::Type) -> Self {
        match ty {
            syn::Type::Reference(ref_ty) => {
                if ref_ty.lifetime.is_some() {
                    unimplemented!("reference type has a lifetime");
                }
                if ref_ty.mutability.is_some() {
                    Type::RefMut(Box::new(Self::from(*ref_ty.elem)))
                } else {
                    Type::Ref(Box::new(Self::from(*ref_ty.elem)))
                }
            }
            syn::Type::Array(_) => unimplemented!("Array"),
            syn::Type::BareFn(_) => unimplemented!("BareFn"),
            syn::Type::Group(_) => unimplemented!("Group"),
            syn::Type::ImplTrait(_) => unimplemented!("ImplTrait"),
            syn::Type::Infer(_) => unimplemented!("Infer"),
            syn::Type::Macro(_) => unimplemented!("Macro"),
            syn::Type::Never(_) => unimplemented!("Never"),
            syn::Type::Paren(_) => unimplemented!("Paren"),
            syn::Type::Path(type_path) => {
                if type_path.qself.is_some() {
                     unimplemented!("path contains qself");
                }
                if let Some(name) = type_path.path.get_ident() {
                    let name = name.to_string();
                    if PRIMITIVE_TYPES.contains(&name.as_str()) {
                        Type::Primitive(name)
                    } else {
                        Type::Struct { name, args: vec![] }
                    }
                } else {
                    if type_path.path.segments.len() != 1 {
                        unimplemented!();
                    }
                    let seg = type_path.path.segments.into_iter().next()
                        .unwrap();
                    let name = seg.ident.to_string();
                    let args = match seg.arguments {
                        syn::PathArguments::None => unreachable!(),
                        syn::PathArguments::Parenthesized(_) => unreachable!(),
                        syn::PathArguments::AngleBracketed(args) => {
                            args.args.into_iter().map(|arg| match arg {
                                syn::GenericArgument::Type(ty) => {
                                    Box::new(Type::from(ty))
                                }
                                _ => unimplemented!()
                            }).collect::<Vec<_>>()
                        }
                    };
                    Type::Struct { name, args }
                }
            },
            syn::Type::Ptr(_) => unimplemented!("Ptr"),
            syn::Type::Slice(_) => unimplemented!("Slice"),
            syn::Type::TraitObject(_) => unimplemented!("TraitObject"),
            syn::Type::Tuple(_) => unimplemented!("Tuple"),
            syn::Type::Verbatim(_) => unimplemented!("Verbatim"),
            _ => unimplemented!("_"),
        }
    }
}

impl Type {
    pub fn from_return_type(ty: syn::ReturnType) -> Option<Self> {
        match ty {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(Self::from(*ty)),
        }
    }

    pub fn update_struct(&mut self, struct_name: &str) {
        match self {
            Type::Primitive(_) | Type::Buffer(_) | Type::Enum { .. } => {}
            Type::Struct { ref mut name, ref mut args } => {
                if name == "Self" {
                    *name = struct_name.to_string();
                }
                for arg_ty in args {
                    arg_ty.update_struct(struct_name);
                }
            }
            Type::Ref(ty) => {
                ty.update_struct(struct_name);
            }
            Type::RefMut(ty) => {
                ty.update_struct(struct_name);
            }
        }
    }
}
