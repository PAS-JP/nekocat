use super::prelude::*;

fn tk_to_sql_string(ts_opt: &Option<proc_macro2::TokenStream>) -> String {
    match ts_opt {
        Some(ts) => {
            if let Ok(litstr) = syn::parse2::<LitStr>(ts.clone()) {
                return litstr.value();
            }

            if let Ok(lit) = syn::parse2::<Lit>(ts.clone())
                && let Lit::Str(ls) = lit
            {
                return ls.value();
            }

            let mut s = ts.to_string();
            if (s.starts_with('"') && s.ends_with('"'))
                || (s.starts_with('\'') && s.ends_with('\''))
            {
                s = s[1..s.len() - 1].to_string();
            }
            s
        }
        None => panic!("sql must be provided example: #[opt(sql = \"TEXT PRIMARY KEY\")]"),
    }
}

pub fn up_migrations(input: &DeriveInput) -> TokenStream {
    let input_clone = input.clone();
    let struct_ident = get_struct_name(input);
    let impl_block = get_impl(input);
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let table_name = struct_ident.to_string().to_lowercase();
    let fields_sql = fields
        .iter()
        .map(|field| {
            let field_name = field
                .ident
                .as_ref()
                .expect("field name must be set")
                .to_string();
            let Opt { sql, .. } = get_opt(&field.attrs);
            let sql = tk_to_sql_string(&sql);
            format!("{field_name} {sql}")
        })
        .collect::<Vec<String>>()
        .join(", ");
    let temporary_table_sql =
        format!("CREATE TEMPORARY TABLE IF NOT EXISTS {table_name} ({fields_sql})");
    let table_sql = format!("CREATE TABLE IF NOT EXISTS {table_name} ({fields_sql})");

    quote! {
        impl #impl_block {
            pub async fn up_migrations_temporary_table_if_not_exists(
                client: &tokio_postgres::Client
            ) -> Result<u64, tokio_postgres::Error> {
                client.execute(#temporary_table_sql,  &[]).await
            }
            pub async fn up_migrations_table_if_not_exists(
                client: &tokio_postgres::Client
            ) -> Result<u64, tokio_postgres::Error> {
                client.execute(#table_sql,  &[]).await
            }
        }
    }
}
