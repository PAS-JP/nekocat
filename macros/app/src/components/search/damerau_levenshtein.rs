use super::prelude::*;

pub fn search_damerau_levenshtein(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let lev_ident = format_ident!("levenshtein_{field_ident}");
    let impl_block = get_impl(input);

    quote! {
        impl #impl_block {
            pub fn #lev_ident(&self, s2: impl Into<String>) -> f32 {
                use core::cmp::min;
                let s2 = s2.into();
                let s1: String = self.#field_ident.clone().into();
                let s1_chars: Vec<char> = s1.chars().collect();
                let s2_chars: Vec<char> = s2.chars().collect();
                let len1 = s1_chars.len();
                let len2 = s2_chars.len();

                if len1 == 0 && len2 == 0 {
                    return 1.0;
                }

                let mut matrix = vec![vec![0usize; len2 + 1]; len1 + 1];

                for i in 0..=len1 {
                    matrix[i][0] = i;
                }
                for j in 0..=len2 {
                    matrix[0][j] = j;
                }

                for i in 1..=len1 {
                    for j in 1..=len2 {
                        let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };

                        let mut val = min(
                            min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                            matrix[i - 1][j - 1] + cost,
                        );

                        if i > 1
                            && j > 1
                            && s1_chars[i - 1] == s2_chars[j - 2]
                            && s1_chars[i - 2] == s2_chars[j - 1]
                        {
                            val = min(val, matrix[i - 2][j - 2] + cost);
                        }

                        matrix[i][j] = val;
                    }
                }

                let distance = matrix[len1][len2] as f64;
                let max_len = len1.max(len2) as f64;

                let similarity = if max_len == 0.0 {
                    1.0
                } else {
                    1.0 - (distance / max_len)
                };

                let mut result = similarity as f32;
                result.clamp(0.0, 1.0)
            }
        }
    }
}
