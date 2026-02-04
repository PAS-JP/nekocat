pub use super::prelude::*;

pub fn get_attributes(input: &DeriveInput) -> Vec<Attribute> {
    input.attrs.clone()
}
