pub use super::prelude::*;

pub fn get_struct_name(input: &DeriveInput) -> Ident {
    input.ident.clone()
}
