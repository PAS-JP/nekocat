use super::prelude::*;

pub fn search_cosine(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let cosine_ident = format_ident!("search_cosine_{field_ident}");
    let impl_block = get_impl(input);
    let ngrams_ident = format_ident!("cosine_ngrams_{field_ident}");
    quote! {
        impl #impl_block {
            fn #ngrams_ident(s: &str, n: usize) -> Vec<String> {
                let chars: Vec<char> = s.chars().collect();
                if chars.len() < n {
                    return chars.into_iter().map(|c| c.to_string()).collect();
                }
                chars
                    .windows(n)
                    .map(|w| w.iter().cloned().collect::<String>())
                    .collect()
            }

            pub fn #cosine_ident(&self, s2: impl Into<String>, n: usize) -> f32 {
                let s2 = s2.into();
                let n = n.max(1);
                let b1 = Self::#ngrams_ident(&self.#field_ident, n);
                let b2 = Self::#ngrams_ident(&s2, n);

                let mut vec1: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                let mut vec2: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

                for bg in b1 {
                    *vec1.entry(bg).or_insert(0) += 1;
                }
                for bg in b2 {
                    *vec2.entry(bg).or_insert(0) += 1;
                }

                let mut dot: usize = 0;
                for (k, v) in &vec1 {
                    if let Some(v2) = vec2.get(k) {
                        dot += v * v2;
                    }
                }

                let norm1: f64 = vec1.values().map(|v| (*v * *v) as f64).sum::<f64>().sqrt();
                let norm2: f64 = vec2.values().map(|v| (*v * *v) as f64).sum::<f64>().sqrt();

                let cosine: f64 = if norm1 == 0.0 || norm2 == 0.0 {
                    0.0
                } else {
                    (dot as f64) / (norm1 * norm2)
                };

                let mut result = cosine as f32;
                result.clamp(0.0, 1.0)
            }
        }
    }
}
