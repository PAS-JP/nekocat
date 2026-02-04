pub use super::prelude::*;

pub fn is_string(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        tp.path.segments.len() == 1 && tp.path.segments[0].ident == "String"
    } else {
        false
    }
}
