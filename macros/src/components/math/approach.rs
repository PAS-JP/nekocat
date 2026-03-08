use super::import::*;

pub fn math_approach(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let method_name_approach = format_ident!("approach_{field_name}");

    quote! {
        /// Moves the field value towards the target by at most `max_delta`.
        ///
        /// If the difference between the current value and the target is less than
        /// or equal to `max_delta`, the field is set to the target value.
        ///
        /// # Parameters
        /// - `target`: The value to approach.
        /// - `max_delta`: The maximum change applied in one call.
        ///
        /// ```rust
        /// use kenzu::Builder;
        /// use malakoi::Math;
        ///
        /// #[derive(Debug, Builder, Math, Clone, Default)]
        /// pub struct CalcStruct {
        ///     pub usize: usize,
        /// }
        ///
        /// let mut calc_struct = CalcStruct::new().usize::<usize>(10).build().unwrap();
        /// calc_struct.approach_usize(20, 6);
        /// assert_eq!(calc_struct.usize, 16);
        /// calc_struct.approach_usize(20, 6);
        /// assert_eq!(calc_struct.usize, 20);
        /// ```
        pub fn #method_name_approach(&mut self, target: #field_type, max_delta: #field_type) {
            if target >= self.#field_name {
                let delta = target - self.#field_name;
                if delta <= max_delta {
                    self.#field_name = target;
                } else {
                    self.#field_name += max_delta;
                }
            } else {
                let delta = self.#field_name - target;
                if delta <= max_delta {
                    self.#field_name = target;
                } else {
                    self.#field_name -= max_delta;
                }
            }
        }
    }
}
