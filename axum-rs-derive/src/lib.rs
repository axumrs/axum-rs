use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod db;

#[proc_macro_derive(Db, attributes(db))]
pub fn db_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let dm = db::DbMeta::new(&ast);

    let fields_ts = dm.fields_ts();
    let insert_ts = dm.insert_ts();
    let update_ts = dm.update_ts();
    let real_del_ts = dm.real_del_ts();
    let del_restore_ts = dm.del_restore_ts();
    let self_update_ts = dm.self_update_ts();
    let exists_ts = dm.exists_ts();

    let find_by_ts = dm.find_by_ts();
    let find_ts = dm.find_ts();

    let list_filter_ts = dm.list_filter_ts();
    let list_ts = dm.list_ts();

    let list_all_filter_ts = dm.list_all_filter_ts();
    let list_all_ts = dm.list_all_ts();

    // println!("{:?}", dm);
    quote! {
        impl #ident {
            #fields_ts
            #insert_ts
            #update_ts
            #real_del_ts
            #del_restore_ts
            #self_update_ts
            #exists_ts
            #find_ts
            #list_ts
            #list_all_ts
        }

        #find_by_ts
        #list_filter_ts
        #list_all_filter_ts
    }
    .into()
}
