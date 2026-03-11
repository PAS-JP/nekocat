pub use super::prelude::*;

pub fn is_vec(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
            if type_path.qself.is_none()
            && type_path
                .path
                .segments
                .last()
                .is_some_and(|seg| seg.ident == "Vec")
    )
}

pub fn is_vec_u8(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
            if type_path.qself.is_none()
            && type_path.path.segments.last().is_some_and(|seg| {
                seg.ident == "Vec" && 
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    args.args.first().is_some_and(|arg| {
                        matches!(arg, GenericArgument::Type(Type::Path(inner)) if inner.path.is_ident("u8"))
                    })
                } else {
                    false
                }
            })
    )
}