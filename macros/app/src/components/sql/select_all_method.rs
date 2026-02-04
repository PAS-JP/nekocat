use super::prelude::*;

pub fn sql_select_all_method(
    input: &DeriveInput,
    fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>,
) -> TokenStream {
    let struct_ident = get_struct_name(input);
    let table_name = struct_ident.to_string().to_lowercase();
    let table_name = format!("\"{table_name}\"");
    let impl_block = get_impl(input);
    let columns_vec: Vec<String> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();
    let columns = columns_vec.join(", ");
    let sql_select_all = format!("SELECT {columns} FROM {table_name}");

    quote! {
        impl #impl_block {
            pub async fn sql_select_all(
                &self,
                client: &tokio_postgres::Client
            ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
                client.query(#sql_select_all, &[]).await
            }
        }
    }
}
