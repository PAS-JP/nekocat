use super::prelude::*;

pub fn search_field_methods(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_type = &field.ty;
    if !(is_string(field_type) || is_str_ref(field_type)) {
        return TokenStream::new();
    }

    let cosine = search_cosine(input, field);
    let damerau_levenshtein = search_damerau_levenshtein(input, field);
    let jaccard = search_jaccard(input, field);
    let jaro = search_jaro(input, field);
    let lcs = search_lcs(input, field);

    quote! {
        #cosine
        #damerau_levenshtein
        #jaccard
        #jaro
        #lcs
    }
}
