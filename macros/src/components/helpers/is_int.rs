pub use super::prelude::*;

pub fn is_int(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path)
            if type_path.qself.is_none()
            && type_path
                .path
                .segments
                .last()
                .is_some_and(|seg| matches!(
                    &seg.ident,
                    id if id == "i8"
                        || id == "i16"
                        || id == "i32"
                        || id == "i64"
                        || id == "i128"
                        || id == "isize"
                        || id == "u8"
                        || id == "u16"
                        || id == "u32"
                        || id == "u64"
                        || id == "u128"
                        || id == "usize"
                ))
    )
}
