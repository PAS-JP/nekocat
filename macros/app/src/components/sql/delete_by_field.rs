use super::prelude::*;

pub fn sql_delete_by_field(input: &DeriveInput, field: &Field) -> TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let field_name = ident.to_string();
    let struct_ident = get_struct_name(input);
    let table_name = struct_ident.to_string().to_lowercase();
    let table_name = format!("\"{table_name}\"");
    let impl_block = get_impl(input);
    let del_fn = format_ident!("sql_delete_by_{field_name}");
    let sql_delete_by = format!("DELETE FROM {table_name} WHERE {field_name} = $1");

    quote! {
        impl #impl_block {
            pub async fn #del_fn(
                &self,
                client: &tokio_postgres::Client
            ) -> Result<u64, tokio_postgres::Error> {
                client.execute(#sql_delete_by, &[&self.#ident]).await
            }
        }
    }
}
