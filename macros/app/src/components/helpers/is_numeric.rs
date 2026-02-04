pub use super::prelude::*;

pub fn is_numeric(ty: &Type) -> bool {
    if let Type::Path(tp) = ty
        && tp.path.segments.len() == 1
    {
        let id = tp.path.segments[0].ident.to_string();
        return matches!(
            id.as_str(),
            "i8" | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "isize"
                | "usize"
                | "f32"
                | "f64"
        );
    }
    false
}
