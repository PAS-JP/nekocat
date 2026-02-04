pub use super::prelude::*;

pub fn is_float(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
            if type_path.qself.is_none()
            && type_path
                .path
                .segments
                .last()
                .is_some_and(|seg| matches!(&seg.ident, id if id == "f32" || id == "f64"))
    )
}
