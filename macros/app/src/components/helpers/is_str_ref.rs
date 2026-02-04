pub use super::prelude::*;

pub fn is_str_ref(ty: &Type) -> bool {
    if let Type::Reference(r) = ty
        && let Type::Path(tp) = &*r.elem
    {
        return tp.path.is_ident("str");
    }
    false
}
