use super::prelude::*;

pub fn search_lcs(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let lcs_ident = format_ident!("lcs_{}", field_ident);
    let impl_block = get_impl(input);

    quote! {
        impl #impl_block {
            pub fn #lcs_ident(&self, s2: String) -> f32 {
                use std::cmp::max;

                let s1: String = self.#field_ident.to_string();
                let a: Vec<char> = s1.chars().collect();
                let b: Vec<char> = s2.chars().collect();

                let n = a.len();
                let m = b.len();

                if n == 0 || m == 0 {
                    return 0.0;
                }

                let mut matrix: Vec<Vec<usize>> = vec![vec![0_usize; m + 1]; n + 1];

                for i in 1..=n {
                    for j in 1..=m {
                        if a[i - 1] == b[j - 1] {
                            matrix[i][j] = matrix[i - 1][j - 1] + 1;
                        } else {
                            matrix[i][j] = max(matrix[i - 1][j], matrix[i][j - 1]);
                        }
                    }
                }

                let lcs_length = matrix[n][m] as f32;
                let max_len = max(n, m) as f32;

                let normalized_score = if max_len == 0.0 {
                    0.0
                } else {
                    lcs_length / max_len
                };

                let mut result = normalized_score;
                result.clamp(0.0, 1.0)
            }
        }
    }
}
