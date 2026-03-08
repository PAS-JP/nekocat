use super::prelude::*;

pub fn search_jaccard(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let jaccard_ident = format_ident!("jaccard_{field_ident}");
    let impl_block = get_impl(input);
    let ngrams_ident = format_ident!("jaccard_ngrams_{field_ident}");

    quote! {
        impl #impl_block {
            fn #ngrams_ident(s: &str, n: usize) -> std::collections::HashSet<String> {
                let chars: Vec<char> = s.chars().collect();

                if chars.is_empty() || n == 0 {
                    return std::collections::HashSet::new();
                }

                let n = n.min(chars.len());

                if n == 1 {
                    return chars.into_iter().map(|c| c.to_string()).collect();
                }

                chars
                    .windows(n)
                    .map(|w| w.iter().cloned().collect::<String>())
                    .collect::<std::collections::HashSet<String>>()
            }

            pub fn #jaccard_ident(&self, s2: impl Into<String>, n: usize) -> f32 {
                let s2 = s2.into();
                let s1: String = self.#field_ident.clone().into();
                let n = n.max(1);

                let set1 = Self::#ngrams_ident(&s1, n);
                let set2 = Self::#ngrams_ident(&s2, n);

                let intersection = set1.intersection(&set2).count() as f64;
                let union = set1.union(&set2).count() as f64;

                let similarity = if union == 0.0 {
                    1.0
                } else {
                    intersection / union
                };

                let mut result = similarity as f32;
                result.clamp(0.0, 1.0)
            }
        }
    }
}
