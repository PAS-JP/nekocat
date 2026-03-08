use super::prelude::*;

pub fn search_jaro(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let jaro_ident = format_ident!("jaro_{field_ident}");
    let impl_block = get_impl(input);

    quote! {
        impl #impl_block {
            pub fn #jaro_ident(&self, s2: impl Into<String>, prefix_size: usize) -> f32 {
                let s2 = s2.into();
                let s1: String = self.#field_ident.clone().into();
                let s1_chars: Vec<char> = s1.chars().collect();
                let s2_chars: Vec<char> = s2.chars().collect();
                let s1_len = s1_chars.len();
                let s2_len = s2_chars.len();

                if s1_len == 0 && s2_len == 0 {
                    return 1.0;
                }
                if s1_len == 0 || s2_len == 0 {
                    return 0.0;
                }

                let max_dist = ((s1_len.max(s2_len)) / 2).saturating_sub(1);

                let mut s2_flags = vec![false; s2_len];
                let mut s1_matches: Vec<char> = Vec::new();

                for (i, c1) in s1_chars.iter().enumerate() {
                    let start = i.saturating_sub(max_dist);
                    let end = (i + max_dist + 1).min(s2_len);
                    for j in start..end {
                        if !s2_flags[j] && *c1 == s2_chars[j] {
                            s1_matches.push(*c1);
                            s2_flags[j] = true;
                            break;
                        }
                    }
                }

                let matches = s1_matches.len();
                if matches == 0 {
                    return 0.0;
                }

                let mut s2_matches: Vec<char> = Vec::with_capacity(matches);
                for (j, &c2) in s2_chars.iter().enumerate() {
                    if s2_flags[j] {
                        s2_matches.push(c2);
                    }
                }

                let mut transpositions = 0usize;
                for i in 0..matches {
                    if s1_matches[i] != s2_matches[i] {
                        transpositions += 1;
                    }
                }

                let m = matches as f64;
                let t = (transpositions as f64) / 2.0;

                let jaro = (m / s1_len as f64 + m / s2_len as f64 + (m - t) / m) / 3.0;

                let prefix_len = s1_chars
                    .iter()
                    .zip(s2_chars.iter())
                    .take(prefix_size)
                    .take_while(|(c1, c2)| c1 == c2)
                    .count();
                let jaro_winkler = jaro + (prefix_len as f64) * 0.076 * (1.0 - jaro);

                let mut result = jaro_winkler as f32;
                result.clamp(0.0, 1.0)
            }
        }
    }
}
