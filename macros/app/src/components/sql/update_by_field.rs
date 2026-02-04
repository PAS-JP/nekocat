use super::prelude::*;

pub fn sql_update_by_field(
    input: &DeriveInput,
    field: &Field,
    fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>,
) -> TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let field_name = ident.to_string();
    let struct_ident = get_struct_name(input);
    let table_name = struct_ident.to_string().to_lowercase();
    let table_name = format!("\"{table_name}\"");
    let impl_block = get_impl(input);
    let columns_vec: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();
    let set_columns: Vec<String> = columns_vec
        .iter()
        .filter(|c| *c != &field_name)
        .cloned()
        .collect();

    let (set_clause, set_param_tokens) = if set_columns.is_empty() {
        (
            format!("{field_name} = {field_name}"),
            Vec::<proc_macro2::TokenStream>::new(),
        )
    } else {
        let set_parts: Vec<String> = set_columns
            .iter()
            .enumerate()
            .map(|(i, col)| format!("{col} = ${}", i + 1))
            .collect();

        let mut toks = Vec::new();
        for col in &set_columns {
            let tok = fields
                .iter()
                .find(|ff| ff.ident.as_ref().unwrap() == col.as_str())
                .map(|ff| {
                    let id = ff.ident.as_ref().unwrap();
                    quote! { (&self.#id) as &(dyn tokio_postgres::types::ToSql + Sync) }
                })
                .unwrap();
            toks.push(tok);
        }

        (set_parts.join(", "), toks)
    };

    let where_idx = if set_param_tokens.is_empty() {
        1
    } else {
        set_param_tokens.len() + 1
    };

    let mut vec_tokens = set_param_tokens.clone();
    vec_tokens.push(quote! { (&self.#ident) as &(dyn tokio_postgres::types::ToSql + Sync) });

    let sql_update_by =
        format!("UPDATE {table_name} SET {set_clause} WHERE {field_name} = ${where_idx}");
    let upd_fn = format_ident!("sql_update_by_{field_name}");

    quote! {
        impl #impl_block {
            pub async fn #upd_fn(
                &self,
                client: &tokio_postgres::Client
            ) -> Result<u64, tokio_postgres::Error> {
                client.execute(#sql_update_by, &[#(#vec_tokens),*]).await
            }
        }
    }
}
