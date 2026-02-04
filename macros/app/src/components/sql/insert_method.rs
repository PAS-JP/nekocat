use super::prelude::*;

pub fn sql_insert_method(
    input: &DeriveInput,
    fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>,
) -> TokenStream {
    let struct_ident = get_struct_name(input);
    let table_name = struct_ident.to_string().to_lowercase();
    let table_name = format!("\"{table_name}\"");
    let impl_block = get_impl(input);
    let insert_params_tokens: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap();
            quote! { (&self.#ident) as &(dyn tokio_postgres::types::ToSql + Sync) }
        })
        .collect();
    let columns_vec: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();
    let columns = columns_vec.join(", ");
    let placeholders_vec: Vec<String> = (1..=columns_vec.len()).map(|i| format!("${i}")).collect();
    let placeholders = placeholders_vec.join(", ");
    let sql_insert = format!("INSERT INTO {table_name} ({columns}) VALUES ({placeholders})");

    quote! {
        impl #impl_block {
            pub async fn sql_insert(
                &self,
                client: &tokio_postgres::Client
            ) -> Result<u64, tokio_postgres::Error> {
                client.execute(#sql_insert, &[#(#insert_params_tokens),*]).await
            }
        }
    }
}
