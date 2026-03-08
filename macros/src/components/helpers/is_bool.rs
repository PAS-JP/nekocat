pub use super::prelude::*;

pub fn is_bool(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && type_path.qself.is_none()
        && let Some(seg) = type_path.path.segments.last()
    {
        seg.ident == "bool"
    } else {
        false
    }
}
