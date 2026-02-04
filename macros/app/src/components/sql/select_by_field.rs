use super::prelude::*;

pub fn sql_select_by_field(
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
    let columns = columns_vec.join(", ");
    let sel_fn = format_ident!("sql_select_by_{field_name}");
    let sql_select_by = format!("SELECT {columns} FROM {table_name} WHERE {field_name} = $1");

    quote! {
        impl #impl_block {
            pub async fn #sel_fn(
                &self,
                client: &tokio_postgres::Client
            ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
                client.query(#sql_select_by, &[&self.#ident]).await
            }
        }
    }
}
