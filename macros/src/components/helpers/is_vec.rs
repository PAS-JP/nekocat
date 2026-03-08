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
