use super::prelude::*;

pub fn sql_app(input: &DeriveInput) -> TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let mut methods = Vec::new();

    for field in fields {
        let tks = vec![
            sql_select_by_field(input, field, fields),
            sql_update_by_field(input, field, fields),
            sql_delete_by_field(input, field),
        ];

        methods.extend(tks)
    }

    let sql_insert_method = sql_insert_method(input, fields);
    let sql_select_all = sql_select_all_method(input, fields);
    let up_migrations = up_migrations(input);

    quote! {
       #sql_select_all
       #sql_insert_method
       #up_migrations
       #(#methods)*
    }
}
