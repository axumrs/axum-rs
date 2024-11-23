use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, Ident, LitStr,
};

#[derive(Debug)]
pub(crate) struct DbField {
    pub(crate) name: Ident,
    pub(crate) ty: syn::Type,
    pub(crate) skip_update: bool,
    pub(crate) skip_insert: bool,
    pub(crate) find: bool,
    pub(crate) find_opt: bool,
    pub(crate) list: bool,
    pub(crate) list_opt: bool,
    pub(crate) find_opt_like: bool,
    pub(crate) list_opt_like: bool,
    pub(crate) find_opt_between: bool,
    pub(crate) list_opt_between: bool,
    pub(crate) exists: bool,
}

#[derive(Debug)]
pub(crate) struct DbMeta {
    pub(crate) table: String,
    pub(crate) pk: String,
    pub(crate) is_view: bool,
    pub(crate) ident: Ident,
    pub(crate) del_field: Option<String>,
    pub(crate) fields: Vec<DbField>,
}

pub(crate) struct DbMetaParser {
    pub(crate) table: Option<LitStr>,
    pub(crate) pk: Option<LitStr>,
    pub(crate) del_filed: Option<LitStr>,
    pub(crate) is_view: bool,
}

impl std::default::Default for DbMetaParser {
    fn default() -> Self {
        Self {
            table: None,
            pk: None,
            is_view: false,
            del_filed: None,
        }
    }
}

impl DbMetaParser {
    fn parse(&mut self, tokens: &proc_macro2::TokenStream) -> TokenStream {
        let parser = syn::meta::parser(|mt| {
            if mt.path.is_ident("table") {
                self.table = Some(mt.value()?.parse()?);
                return Ok(());
            }
            if mt.path.is_ident("pk") {
                self.pk = Some(mt.value()?.parse()?);
                return Ok(());
            }
            if mt.path.is_ident("is_view") {
                self.is_view = true;
                return Ok(());
            }

            if mt.path.is_ident("del_field") {
                self.del_filed = Some(mt.value()?.parse()?);
                return Ok(());
            }

            Ok(())
        });

        let tokens = tokens.to_owned().into();
        parse_macro_input!(tokens with parser);

        quote! {}.into()
    }
}

impl DbMeta {
    fn all_fields(&self) -> Vec<Ident> {
        self.fields.iter().map(|f| f.name.clone()).collect()
    }
    fn insert_fields(&self) -> Vec<Ident> {
        self.fields
            .iter()
            .filter(|f| f.skip_insert == false)
            .map(|f| f.name.clone())
            .collect()
    }
    fn update_fields(&self) -> Vec<Ident> {
        self.fields
            .iter()
            .filter(|f| f.skip_update == false)
            .map(|f| f.name.clone())
            .collect()
    }
    fn find_by_fields(&self) -> (Vec<Ident>, Vec<syn::Type>) {
        let mut ids = vec![];
        let mut tys = vec![];
        for f in self.fields.iter().filter(|f| f.find == true) {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
        }
        (ids, tys)
    }

    fn find_filter_fields(&self) -> (Vec<Ident>, Vec<syn::Type>, Vec<bool>) {
        let mut ids = vec![];
        let mut tys = vec![];
        let mut ols = vec![];

        for f in self
            .fields
            .iter()
            .filter(|f| f.find_opt == true && f.find_opt_between == false)
        {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
            ols.push(f.find_opt_like);
        }
        (ids, tys, ols)
    }

    fn find_filter_between_fields(&self) -> (Vec<Ident>, Vec<syn::Type>) {
        let mut ids = vec![];
        let mut tys = vec![];

        for f in self
            .fields
            .iter()
            .filter(|f| f.find_opt == true && f.find_opt_between == true)
        {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
        }
        (ids, tys)
    }

    pub(crate) fn list_filter_fields(&self) -> (Vec<Ident>, Vec<syn::Type>) {
        let mut ids = vec![];
        let mut tys = vec![];

        for f in self.fields.iter().filter(|f| f.list == true) {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
        }
        (ids, tys)
    }
    pub(crate) fn list_filter_fields_opt(&self) -> (Vec<Ident>, Vec<syn::Type>, Vec<bool>) {
        let mut ids = vec![];
        let mut tys = vec![];
        let mut ols = vec![];

        for f in self
            .fields
            .iter()
            .filter(|f| f.list_opt == true && f.list_opt_between == false)
        {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
            ols.push(f.list_opt_like);
        }
        (ids, tys, ols)
    }
    pub(crate) fn list_filter_fields_opt_between(&self) -> (Vec<Ident>, Vec<syn::Type>) {
        let mut ids = vec![];
        let mut tys = vec![];

        for f in self
            .fields
            .iter()
            .filter(|f| f.list_opt == true && f.list_opt_between == true)
        {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
        }
        (ids, tys)
    }

    fn pk_ident(&self) -> Ident {
        Ident::new(&self.pk, self.ident.span())
    }
    fn pk_type(&self) -> syn::Type {
        self.fields
            .iter()
            .find(|f| f.name.to_string() == self.pk)
            .take()
            .unwrap()
            .ty
            .clone()
    }
    fn self_update_fields(&self) -> (Vec<Ident>, Vec<syn::Type>) {
        let mut ids = vec![];
        let mut tys = vec![];
        for f in self
            .fields
            .iter()
            .filter(|f| f.name.to_string() != self.pk)
            .filter(|f| f.skip_update == false)
        {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
        }
        (ids, tys)
    }

    fn exists_filter_fields(&self) -> (Vec<Ident>, Vec<syn::Type>) {
        let mut ids = vec![];
        let mut tys = vec![];

        for f in self
            .fields
            .iter()
            .filter(|f| f.exists && f.name.to_string() != self.pk)
        {
            ids.push(f.name.clone());
            tys.push(f.ty.clone());
        }
        (ids, tys)
    }

    fn parse_fields(ast: &DeriveInput) -> &Punctuated<Field, Comma> {
        if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }, ..),
            ..
        }) = &ast.data
        {
            return named;
        }
        unreachable!()
    }
    pub(crate) fn new(ast: &DeriveInput) -> Self {
        let ident = ast.ident.clone();
        let ident_str = ident.to_string();
        let table = helper::gen_table_name(&ident_str);

        let mut dmp = DbMetaParser::default();
        let mut dm = Self {
            table,
            pk: "id".into(),
            is_view: false,
            ident,
            del_field: None,
            fields: vec![],
        };

        for a in ast.attrs.iter() {
            if let syn::Meta::List(syn::MetaList { path, tokens, .. }) = &a.meta {
                if let Some(seg) = path.segments.first() {
                    if seg.ident == "db" {
                        dmp.parse(tokens);
                        if let Some(v) = &dmp.table {
                            dm.table = v.token().to_string();
                        }
                        if let Some(v) = &dmp.pk {
                            dm.pk = v.token().to_string();
                        }
                        if let Some(v) = &dmp.del_filed {
                            dm.del_field = Some(v.token().to_string());
                        }
                        dm.is_view = dmp.is_view;
                    }
                }
            }
        }

        let meta_fields = Self::parse_fields(ast);
        let mut fields = vec![];

        for f in meta_fields {
            let name = f.ident.clone();
            let ty = f.ty.clone();
            let attrs = f
                .attrs
                .clone()
                .into_iter()
                .filter(|a| a.path().is_ident("db"))
                .collect::<Vec<_>>();

            let mut db_field = DbField {
                name: name.unwrap(),
                ty,
                skip_update: false,
                skip_insert: false,
                find: false,
                find_opt: false,
                list: false,
                list_opt: false,
                find_opt_like: false,
                list_opt_like: false,
                find_opt_between: false,
                list_opt_between: false,
                exists: false,
            };

            for a in attrs.iter() {
                a.parse_nested_meta(|mt| {
                    if mt.path.is_ident("skip_update") {
                        db_field.skip_update = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("skip_insert") {
                        db_field.skip_insert = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("find") {
                        db_field.find = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("find_opt") {
                        db_field.find_opt = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("list") {
                        db_field.list = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("list_opt") {
                        db_field.list_opt = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("find_opt_like") {
                        db_field.find_opt_like = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("list_opt_like") {
                        db_field.list_opt_like = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("find_opt_between") {
                        db_field.find_opt_between = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("list_opt_between") {
                        db_field.list_opt_between = true;
                        return Ok(());
                    }
                    if mt.path.is_ident("exists") {
                        db_field.exists = true;
                        return Ok(());
                    }

                    Ok(())
                })
                .unwrap();
            }
            fields.push(db_field);
        }
        dm.fields = fields;
        dm
    }

    pub(crate) fn fields_ts(&self) -> proc_macro2::TokenStream {
        let all_fields = self
            .all_fields()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let insert_fields = self
            .insert_fields()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let update_fields = self
            .update_fields()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let table = self.table.clone();
        let all_fields_str = all_fields
            .iter()
            .map(|f| format!("{:?}", f))
            .collect::<Vec<_>>()
            .join(",");
        quote! {
            pub fn table() -> String {
                #table.to_string()
            }
            pub fn all_fields()-> Vec<&'static str> {
                vec![#(#all_fields),*]
            }

            pub fn insert_fileds() -> Vec<&'static str> {
                vec![#(#insert_fields),*]
            }

            pub fn update_fields() -> Vec<&'static str> {
                vec![#(#update_fields),*]
            }

            pub fn fields()->String {
                #all_fields_str.to_string()
            }
        }
    }

    pub(crate) fn insert_ts(&self) -> proc_macro2::TokenStream {
        if self.is_view {
            return quote! {};
        }

        let fields = self.insert_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let fields_str = fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>()
            .join(",");
        let table = &self.table;
        let sql = format!("INSERT INTO {:?} ({}) ", table, &fields_str);
        let pk = self.pk_ident();
        quote! {
            pub async fn insert<'a>(&self, e: impl ::sqlx::PgExecutor<'a>)-> ::sqlx::Result<String> {
                let id = self.#pk.clone();
                let sql = #sql;
                let mut q = ::sqlx::QueryBuilder::new(sql);
                q.push_values(&[self], |mut b, m| {
                    #(b.push_bind(&m.#fields);)*
                });
                tracing::debug!("{}",q.sql());
                q.build().execute(e).await?;
                Ok(id)
            }
        }
    }

    pub(crate) fn update_ts(&self) -> proc_macro2::TokenStream {
        if self.is_view {
            return quote! {};
        }
        let fields = self.update_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let fields_str = fields
            .iter()
            .map(|f| format!("{:?} = ", f.to_string()))
            .collect::<Vec<_>>();
        let fields_com = fields
            .iter()
            .enumerate()
            .map(|(idx, _)| format!("{}", if idx < fields.len() - 1 { ", " } else { "" }))
            .collect::<Vec<_>>();

        let table = self.table.clone();
        let sql = format!("UPDATE {:?} SET ", &table);
        let pk = self.pk_ident();
        let pk_str = pk.to_string();
        quote! {
            pub async fn update<'a>(&self, e: impl  ::sqlx::PgExecutor<'a>) -> ::sqlx::Result<u64> {
                let mut q = ::sqlx::QueryBuilder::new(#sql);
                #(
                    q.push(#fields_str)
                    .push_bind(&self.#fields)
                    .push(#fields_com);
                )*
                let where_str = format!(" WHERE {:?} =", #pk_str);
                q.push(&where_str).push_bind(&self.#pk);

                tracing::debug!("{}",q.sql());
                let aff = q.build().execute(e).await?.rows_affected();

                Ok(aff)
            }
        }
    }

    pub(crate) fn find_by_ts(&self) -> proc_macro2::TokenStream {
        let fields = self.all_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let (find_by_origin_fields, find_by_types) = self.find_by_fields();
        let (find_filter_fields, find_filter_types, _) = self.find_filter_fields();
        let (find_origin_filter_between_fields, find_filter_between_types) =
            self.find_filter_between_fields();
        let (find_by_is_empty, find_filter_is_empty, find_filter_between_is_empty) = (
            find_by_origin_fields.is_empty(),
            find_filter_fields.is_empty(),
            find_origin_filter_between_fields.is_empty(),
        );
        if find_by_is_empty && find_filter_is_empty && find_filter_between_is_empty {
            return quote! {};
        }

        let ident = self.ident.clone();
        let ident_str = ident.to_string();

        let find_ident_str = format!("{}FindFilter", &ident_str);
        let find_ident = Ident::new(&find_ident_str, ident.span().clone());

        let find_by_ident_str = format!("{}FindBy", &ident_str);
        let find_by_ident = Ident::new(&find_by_ident_str, ident.span());
        let find_by_fields = find_by_origin_fields
            .iter()
            .map(|f| helper::gen_entity_ident(f.to_owned()))
            .collect::<Vec<_>>();

        let find_filter_between_fields_str = find_origin_filter_between_fields
            .iter()
            .map(|f| {
                format!(
                    "{}FindBetween{}",
                    &ident_str,
                    helper::gen_entity_ident(f.to_owned()).to_string()
                )
            })
            .collect::<Vec<_>>();
        let find_filter_between_fields = find_filter_between_fields_str
            .iter()
            .map(|s| Ident::new(s, ident.span()))
            .collect::<Vec<_>>();
        let find_filter_between_fields_snake = find_filter_between_fields_str
            .iter()
            .map(|s| helper::gen_table_name(s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();

        let mut ts = proc_macro2::TokenStream::new();

        if !find_by_is_empty {
            ts.extend(quote! {
                pub enum #find_by_ident {
                    #( #find_by_fields(#find_by_types), )*
                }
            });
        }

        if !find_filter_between_is_empty {
            ts.extend(quote! {
                 #(
                pub struct #find_filter_between_fields {
                    pub start: #find_filter_between_types,
                    pub end: #find_filter_between_types,
                }
            )*
            });
        }
        let struct_str = if find_by_is_empty {
            quote! {
                pub struct #find_ident {
                    #( pub #find_filter_fields: ::std::option::Option<#find_filter_types>, )*
                    #( pub #find_filter_between_fields_snake:  ::std::option::Option<#find_filter_between_fields>,)*
                }
            }
        } else {
            quote! {
                   pub struct #find_ident {
                        pub by: #find_by_ident,

                        #( pub #find_filter_fields: ::std::option::Option<#find_filter_types>, )*
                        #( pub #find_filter_between_fields_snake:  ::std::option::Option<#find_filter_between_fields>,)*
                }
            }
        };
        ts.extend(struct_str);
        ts
    }

    pub(crate) fn find_ts(&self) -> proc_macro2::TokenStream {
        let fields = self.all_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let (find_by_origin_fields, _) = self.find_by_fields();
        let (find_filter_fields, _, find_filter_opt_like) = self.find_filter_fields();
        let (find_origin_filter_between_fields, _) = self.find_filter_between_fields();

        if find_by_origin_fields.is_empty()
            && find_filter_fields.is_empty()
            && find_origin_filter_between_fields.is_empty()
        {
            return quote! {};
        }

        let ident = self.ident.clone();
        let ident_str = ident.to_string();
        let find_by_ident_str = format!("{}FindBy", &ident_str);
        let find_by_ident = Ident::new(&find_by_ident_str, ident.span());
        let find_ident_str = format!("{}FindFilter", &ident_str);
        let find_ident = Ident::new(&find_ident_str, ident.span());

        let find_by_fields = find_by_origin_fields
            .iter()
            .map(|f| helper::gen_entity_ident(f.to_owned()))
            .collect::<Vec<_>>();
        let find_by_fields_str = find_by_origin_fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();

        let find_filter_fields_str = find_filter_fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();

        let fields_str_arr = fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();
        let fields_str = fields_str_arr.join(", ");
        let sql = format!("SELECT {} FROM {:?} WHERE 1=1", &fields_str, &self.table);

        let find_origin_filter_between_fields_str = find_origin_filter_between_fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let find_filter_between_fields_str = find_origin_filter_between_fields
            .iter()
            .map(|f| {
                format!(
                    "{}FindBetween{}",
                    &ident_str,
                    helper::gen_entity_ident(f.to_owned()).to_string()
                )
            })
            .collect::<Vec<_>>();

        let find_filter_between_fields_snake = find_filter_between_fields_str
            .iter()
            .map(|s| helper::gen_table_name(s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();

        let by_ts = if find_by_origin_fields.is_empty() {
            quote! {}
        } else {
            quote! {
                  match &f.by {
                        #( #find_by_ident::#find_by_fields(v) => { q.push(format!(" AND {} = ", &#find_by_fields_str)).push_bind(v); }, )*
                    };
            }
        };

        quote! {
            pub async fn find<'a>(e: impl  ::sqlx::PgExecutor<'a>, f:&#find_ident) -> ::sqlx::Result<::std::option::Option<Self>> {

                let mut q = ::sqlx::QueryBuilder::new(#sql);
                #by_ts

                #(
                    if let Some(v) = &f.#find_filter_fields {

                        if #find_filter_opt_like {
                            let parm = format!("%{}%", v);
                            q.push(format!(" AND {} ILIKE ", &#find_filter_fields_str)).push_bind(parm);
                        } else {
                              q.push(format!(" AND {} =", &#find_filter_fields_str)).push_bind(v);
                        }
                    }
                )*


                #(
                    if let Some(v) = &f.#find_filter_between_fields_snake {

                        q.push(format!(" AND {:?} BETWEEN ", &#find_origin_filter_between_fields_str)).push_bind(&v.start).push(" AND ").push_bind(&v.end);
                    }
                )*
                tracing::debug!("{}", q.sql());
                q.build_query_as().fetch_optional(e).await
            }
        }
    }

    pub(crate) fn list_filter_ts(&self) -> proc_macro2::TokenStream {
        let fields = self.all_fields();
        if fields.is_empty() {
            return quote! {};
        }
        let (filter_fields, filter_types) = self.list_filter_fields();
        let (filter_fields_opt, filter_types_opt, _) = self.list_filter_fields_opt();
        let (list_origin_filter_between_fields, list_filter_between_types) =
            self.list_filter_fields_opt_between();
        // let (filter_is_empty, filter_opt_is_empty, filter_between_is_empty) = (
        //     filter_fields.is_empty(),
        //     filter_fields_opt.is_empty(),
        //     list_origin_filter_between_fields.is_empty(),
        // );

        // if filter_is_empty && filter_opt_is_empty && filter_between_is_empty {
        //     return quote! {};
        // }

        let ident = self.ident.clone();
        let ident_str = ident.to_string();
        let filter_ident_str = format!("{}ListFilter", &ident_str);
        let filter_ident = Ident::new(&filter_ident_str, ident.span());
        let paginate_ident_str = format!("{}Paginate", &ident_str);
        let paginate_ident = Ident::new(&paginate_ident_str, ident.span());
        let paginate_req_ident_str = format!("{}PaginateReq", &ident_str);
        let paginate_req_ident = Ident::new(&paginate_req_ident_str, ident.span());

        let list_filter_fields_opt_between_str = list_origin_filter_between_fields
            .iter()
            .map(|f| {
                format!(
                    "{}ListBetween{}",
                    &ident_str,
                    helper::gen_entity_ident(f.to_owned()).to_string()
                )
            })
            .collect::<Vec<_>>();
        let list_filter_fields_opt_between = list_filter_fields_opt_between_str
            .iter()
            .map(|s| Ident::new(s, ident.span()))
            .collect::<Vec<_>>();
        let list_filter_fields_opt_between_snake = list_filter_fields_opt_between_str
            .iter()
            .map(|s| helper::gen_table_name(s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();
        let default_page_size_ident_str = format!("{}_DEFAULT_PAGE_SIZE", ident_str.to_uppercase());
        let default_page_size_ident = Ident::new(&default_page_size_ident_str, ident.span());

        quote! {
             const #default_page_size_ident:u32 = 30;

              #(
                  #[derive(Debug)]
                    pub struct #list_filter_fields_opt_between {
                        pub start: #list_filter_between_types,
                        pub end: #list_filter_between_types,
                    }
                )*

             #[derive(Debug)]
            pub struct #filter_ident {
                pub pq:#paginate_req_ident,
                pub order:Option<String>,
                #( pub #filter_fields: #filter_types, )*
                #( pub #filter_fields_opt: ::std::option::Option<#filter_types_opt>, )*
                 #( pub #list_filter_fields_opt_between_snake:  ::std::option::Option<#list_filter_fields_opt_between>,)*
            }
               #[derive(Debug)]
            pub struct #paginate_req_ident {
                  pub page:u32,
                pub page_size:u32,
            }
            impl #paginate_req_ident {
                pub fn new(page:u32) -> Self {
                    Self {page, page_size:#default_page_size_ident}
                }
                pub fn page_size(&self) -> i64 {
                    self.page_size as i64
                }
                pub fn offset(&self) -> i64 {
                    (self.page_size * self.page ) as i64
                }
            }

               #[derive(Debug,::serde::Serialize)]
            pub struct #paginate_ident {
                pub total:u32,
                pub total_page:u32,
                pub page:u32,
                pub page_size:u32,
                pub data: Vec<#ident>,
            }
            impl #paginate_ident {
                pub fn new(total:u32, page:u32, page_size:u32, data:Vec<#ident>) -> Self {
                    let total_page = f64::ceil(total as f64/page_size as f64) as u32;
                    Self {
                        total,
                        page,
                        total_page,
                        page_size,
                        data,
                    }
                }
                pub fn quick(total:i64, p:&#paginate_req_ident, data:Vec<#ident>) -> Self {
                    Self::new(total as u32, p.page, p.page_size, data)
                }
            }
        }
    }

    pub(crate) fn list_ts(&self) -> proc_macro2::TokenStream {
        let fields = self.all_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let (filter_fields, _) = self.list_filter_fields();
        let (filter_fields_opt, _, filter_like_opt) = self.list_filter_fields_opt();
        let (list_origin_filter_between_fields, _) = self.list_filter_fields_opt_between();

        // if filter_fields.is_empty()
        //     && filter_fields_opt.is_empty()
        //     && list_origin_filter_between_fields.is_empty()
        // {
        //     return quote! {};
        // }

        let ident = self.ident.clone();
        let ident_str = ident.to_string();
        let filter_ident_str = format!("{}ListFilter", &ident_str);
        let filter_ident = Ident::new(&filter_ident_str, ident.span());

        let filter_fields_str = filter_fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();
        let filter_fields_opt_str = filter_fields_opt
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();

        let fields_str_arr = fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();
        let fields_str = fields_str_arr.join(", ");
        let sql = format!("SELECT {} FROM {:?} WHERE 1=1", &fields_str, &self.table);
        let sql_count = format!("SELECT COUNT(*) FROM {:?} WHERE 1=1", &self.table);

        let paginate_ident_str = format!("{}Paginate", &ident_str);
        let paginate_ident = Ident::new(&paginate_ident_str, ident.span());
        let pk = self.pk_ident();
        let pk_str = format!("{:?}", pk.to_string());

        let list_origin_filter_between_fields_str = list_origin_filter_between_fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let list_filter_fields_opt_between_str = list_origin_filter_between_fields
            .iter()
            .map(|f| {
                format!(
                    "{}ListBetween{}",
                    &ident_str,
                    helper::gen_entity_ident(f.to_owned()).to_string()
                )
            })
            .collect::<Vec<_>>();

        let list_filter_fields_opt_between_snake = list_filter_fields_opt_between_str
            .iter()
            .map(|s| helper::gen_table_name(s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();

        quote! {
            pub async fn list(p:&::sqlx::PgPool, f:&#filter_ident) -> ::sqlx::Result<#paginate_ident> {
                let mut tx = p.begin().await?;
                let data = Self::list_data(&mut *tx,f).await?;
                let count = Self::list_count(&mut *tx,f).await?;
                tx.commit().await?;
                Ok(#paginate_ident::quick(count,&f.pq,data))
            }
            pub async fn list_data<'a>(e: impl  ::sqlx::PgExecutor<'a>,f:&#filter_ident) -> ::sqlx::Result<Vec<#ident>>{
                let mut q = ::sqlx::QueryBuilder::new(#sql);

                #(

                        q.push(format!(" AND {} = ", &#filter_fields_str)).push_bind(&f.#filter_fields);

                )*

                #(
                    if let Some(v) = &f.#filter_fields_opt {
                        if #filter_like_opt {
                            let param = format!("%{}%", v);
                            q.push(format!(" AND {} ILIKE ", &#filter_fields_opt_str)).push_bind(param);
                        } else {
                            q.push(format!(" AND {} = ", &#filter_fields_opt_str)).push_bind(v);
                        }
                    }
                )*

                #(
                     if let Some(v) = &f.#list_filter_fields_opt_between_snake {

                        q.push(format!(" AND {:?} BETWEEN ", &#list_origin_filter_between_fields_str)).push_bind(&v.start).push(" AND ").push_bind(&v.end);
                    }
                )*

                // 排序
                let order = if let Some(v) = &f.order {
                    v.clone()
                } else {
                    format!("{} DESC", #pk_str)
                };
                let order = format!(" ORDER BY {}", order);
                q.push(&order);

                // 分页
                q.push(" LIMIT ").push_bind(f.pq.page_size()).push(" OFFSET ").push_bind(f.pq.offset());

                tracing::debug!("{}",q.sql());
                q.build_query_as().fetch_all(e).await
            }
            pub async fn list_count<'a>(e: impl  ::sqlx::PgExecutor<'a>,f:&#filter_ident) -> ::sqlx::Result<i64>{
                let mut q = ::sqlx::QueryBuilder::new(#sql_count);

                 #(

                        q.push(format!(" AND {} = ", &#filter_fields_str)).push_bind(&f.#filter_fields);

                )*

                #(
                    if let Some(v) = &f.#filter_fields_opt {
                        if #filter_like_opt {
                            let param = format!("%{}%", v);
                            q.push(format!(" AND {} ILIKE ", &#filter_fields_opt_str)).push_bind(param);
                        } else {
                            q.push(format!(" AND {} = ", &#filter_fields_opt_str)).push_bind(v);
                        }
                    }
                )*

                 #(
                     if let Some(v) = &f.#list_filter_fields_opt_between_snake {

                        q.push(format!(" AND {:?} BETWEEN ", &#list_origin_filter_between_fields_str)).push_bind(&v.start).push(" AND ").push_bind(&v.end);
                    }
                )*

                tracing::debug!("{}",q.sql());

                let count:(i64,)=q.build_query_as().fetch_one(e).await?;
                Ok(count.0)
            }

            pub  fn build_list_query<'a>(mut q: ::sqlx::QueryBuilder<'a, ::sqlx::Postgres>, f:&'a #filter_ident)->::sqlx::QueryBuilder<'a, ::sqlx::Postgres> {


                  #(

                        q.push(format!(" AND {} = ", &#filter_fields_str)).push_bind(&f.#filter_fields);

                )*

                #(
                    if let Some(v) = &f.#filter_fields_opt {
                        if #filter_like_opt {
                            let param = format!("%{}%", v);
                            q.push(format!(" AND {} ILIKE ", &#filter_fields_opt_str)).push_bind(param);
                        } else {
                            q.push(format!(" AND {} = ", &#filter_fields_opt_str)).push_bind(v);
                        }
                    }
                )*

                 #(
                     if let Some(v) = &f.#list_filter_fields_opt_between_snake {

                        q.push(format!(" AND {:?} BETWEEN ", &#list_origin_filter_between_fields_str)).push_bind(&v.start).push(" AND ").push_bind(&v.end);
                    }
                )*

                 tracing::debug!("{}",q.sql());
                q
            }


        }
    }

    pub(crate) fn list_all_filter_ts(&self) -> proc_macro2::TokenStream {
        let fields = self.all_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let (filter_fields, filter_types) = self.list_filter_fields();
        let (filter_fields_opt, filter_types_opt, _) = self.list_filter_fields_opt();
        let (list_origin_filter_between_fields, list_filter_between_types) =
            self.list_filter_fields_opt_between();
        // if filter_fields.is_empty()
        //     && filter_fields_opt.is_empty()
        //     && list_origin_filter_between_fields.is_empty()
        // {
        //     return quote! {};
        // }

        let ident = self.ident.clone();
        let ident_str = ident.to_string();
        let filter_ident_str = format!("{}ListAllFilter", &ident_str);
        let filter_ident = Ident::new(&filter_ident_str, ident.span());

        let list_filter_fields_opt_between_str = list_origin_filter_between_fields
            .iter()
            .map(|f| {
                format!(
                    "{}ListAllBetween{}",
                    &ident_str,
                    helper::gen_entity_ident(f.to_owned()).to_string()
                )
            })
            .collect::<Vec<_>>();
        let list_filter_fields_opt_between = list_filter_fields_opt_between_str
            .iter()
            .map(|s| Ident::new(s, ident.span()))
            .collect::<Vec<_>>();
        let list_filter_fields_opt_between_snake = list_filter_fields_opt_between_str
            .iter()
            .map(|s| helper::gen_table_name(s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();

        quote! {

              #(
                  #[derive(Debug)]
                    pub struct #list_filter_fields_opt_between {
                        pub start: #list_filter_between_types,
                        pub end: #list_filter_between_types,
                    }
                )*

             #[derive(Debug)]
            pub struct #filter_ident {
                pub limit:Option<i64>,
                pub order:Option<String>,
                #( pub #filter_fields: #filter_types, )*
                #( pub #filter_fields_opt: ::std::option::Option<#filter_types_opt>, )*
                 #( pub #list_filter_fields_opt_between_snake:  ::std::option::Option<#list_filter_fields_opt_between>,)*
            }
            impl #filter_ident {
                pub fn limit(&self)->i64 {
                    self.limit.unwrap_or(300)
                }
            }
        }
    }

    pub(crate) fn list_all_ts(&self) -> proc_macro2::TokenStream {
        let fields = self.all_fields();
        if fields.is_empty() {
            return quote! {};
        }

        let (filter_fields, _) = self.list_filter_fields();
        let (filter_fields_opt, _, filter_like_opt) = self.list_filter_fields_opt();
        let (list_origin_filter_between_fields, _) = self.list_filter_fields_opt_between();

        // if filter_fields.is_empty()
        //     && filter_fields_opt.is_empty()
        //     && list_origin_filter_between_fields.is_empty()
        // {
        //     return quote! {};
        // }

        let ident = self.ident.clone();
        let ident_str = ident.to_string();
        let filter_ident_str = format!("{}ListAllFilter", &ident_str);
        let filter_ident = Ident::new(&filter_ident_str, ident.span());
        let filter_fields_str = filter_fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();
        let filter_fields_opt_str = filter_fields_opt
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();

        let fields_str_arr = fields
            .iter()
            .map(|f| format!("{:?}", f.to_string()))
            .collect::<Vec<_>>();
        let fields_str = fields_str_arr.join(", ");
        let sql = format!("SELECT {} FROM {:?} WHERE 1=1", &fields_str, &self.table);

        let pk = self.pk_ident();
        let pk_str = format!("{:?}", pk.to_string());

        let list_origin_filter_between_fields_str = list_origin_filter_between_fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let list_filter_fields_opt_between_str = list_origin_filter_between_fields
            .iter()
            .map(|f| {
                format!(
                    "{}ListAllBetween{}",
                    &ident_str,
                    helper::gen_entity_ident(f.to_owned()).to_string()
                )
            })
            .collect::<Vec<_>>();

        let list_filter_fields_opt_between_snake = list_filter_fields_opt_between_str
            .iter()
            .map(|s| helper::gen_table_name(s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();

        quote! {

            pub async fn list_all<'a>(e: impl  ::sqlx::PgExecutor<'a>,f:&#filter_ident) -> ::sqlx::Result<Vec<#ident>>{
                let mut q = ::sqlx::QueryBuilder::new(#sql);

                #(

                        q.push(format!(" AND {} = ", &#filter_fields_str)).push_bind(&f.#filter_fields);

                )*

                #(
                    if let Some(v) = &f.#filter_fields_opt {
                        if #filter_like_opt {
                            let param = format!("%{}%", v);
                            q.push(format!(" AND {} ILIKE ", &#filter_fields_opt_str)).push_bind(param);
                        } else {
                            q.push(format!(" AND {} = ", &#filter_fields_opt_str)).push_bind(v);
                        }
                    }
                )*

                #(
                     if let Some(v) = &f.#list_filter_fields_opt_between_snake {

                        q.push(format!(" AND {:?} BETWEEN ", &#list_origin_filter_between_fields_str)).push_bind(&v.start).push(" AND ").push_bind(&v.end);
                    }
                )*

                // 排序
                let order = if let Some(v) = &f.order {
                    v.clone()
                } else {
                    format!("{} DESC", #pk_str)
                };
                let order = format!(" ORDER BY {}", order);
                q.push(&order);

                // 限制记录数
                q.push(" LIMIT ").push_bind(f.limit());

                tracing::debug!("{}",q.sql());
                q.build_query_as().fetch_all(e).await
            }
        }
    }

    pub(crate) fn real_del_ts(&self) -> proc_macro2::TokenStream {
        if self.is_view {
            return quote! {};
        }

        let pk = self.pk_ident();
        let pk_str = pk.to_string();
        let pk_ty = self.pk_type();
        let sql = format!("DELETE FROM {:?} WHERE {:?}= $1", &self.table, &pk_str);
        quote! {
             pub async fn real_del<'a>(e: impl  ::sqlx::PgExecutor<'a>, #pk:&#pk_ty)->::sqlx::Result<u64> {
                let aff = ::sqlx::query(#sql).bind(#pk).execute(e).await?.rows_affected();
                 Ok(aff)
             }
        }
    }

    pub(crate) fn del_restore_ts(&self) -> proc_macro2::TokenStream {
        if self.is_view {
            return quote! {};
        }
        let del_field = self.del_field.clone();
        if del_field.is_none() {
            return quote! {};
        }
        let del_field_str = del_field.unwrap();

        let pk = self.pk_ident();
        let pk_str = pk.to_string();
        let pk_ty = self.pk_type();
        let del_sql = format!(
            "UPDATE {:?} SET {:?}=TRUE WHERE {}= $1",
            &self.table, del_field_str, &pk_str
        );
        let restore_sql = format!(
            "UPDATE {:?} SET {:?}=FALSE WHERE {}= $1",
            &self.table, del_field_str, &pk_str
        );
        quote! {
             pub async fn del<'a>(e: impl  ::sqlx::PgExecutor<'a>, #pk:&#pk_ty)->::sqlx::Result<u64> {
                let aff = ::sqlx::query(#del_sql).bind(#pk).execute(e).await?.rows_affected();
                 Ok(aff)
             }
             pub async fn restore<'a>(e: impl  ::sqlx::PgExecutor<'a>, #pk:&#pk_ty)->::sqlx::Result<u64> {
                let aff = ::sqlx::query(#restore_sql).bind(#pk).execute(e).await?.rows_affected();
                 Ok(aff)
             }
        }
    }

    pub(crate) fn self_update_ts(&self) -> proc_macro2::TokenStream {
        if self.is_view {
            return quote! {};
        }

        let (self_update_fields, self_update_types) = self.self_update_fields();
        if self_update_fields.is_empty() {
            return quote! {};
        }
        let self_update_field_idents = self_update_fields
            .iter()
            .map(|f| f.clone())
            .collect::<Vec<_>>();

        let self_update_field_idents_str = self_update_field_idents
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();
        let self_update_field_entity_idents = self_update_field_idents_str
            .iter()
            .map(|f| {
                let s = format!("update_{}", f);
                Ident::new(&s, self.ident.span())
            })
            .collect::<Vec<_>>();
        let pk = self.pk_ident();
        let pk_str = pk.to_string();
        let pk_ty = self.pk_type();
        let table = &self.table;
        quote! {
            #(
                pub async fn #self_update_field_entity_idents<'a>(e: impl  ::sqlx::PgExecutor<'a>, #self_update_field_idents:&#self_update_types, #pk:&#pk_ty)->::sqlx::Result<u64> {
                    let sql = format!("UPDATE {} SET {} = ", #table, #self_update_field_idents_str);
                    let where_str = format!(" WHERE {} = ", #pk_str);
                    let mut q = ::sqlx::QueryBuilder::new(&sql);
                    q.push_bind(#self_update_field_idents)
                    .push(&where_str).push_bind(#pk);
                    let aff = q.build().execute(e).await?.rows_affected();
                    Ok(aff)
                }
            )*
        }
    }

    pub(crate) fn exists_ts(&self) -> proc_macro2::TokenStream {
        let (origin_exists_filter_fields, origin_exists_filter_types) = self.exists_filter_fields();

        if origin_exists_filter_fields.is_empty() {
            return quote! {};
        }
        let ident = self.ident.clone();

        let table = self.table.clone();
        let origin_exists_filter_fields_str = origin_exists_filter_fields
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        let sql = format!("SELECT COUNT(*) FROM {:?} WHERE 1=1", &table);
        let fn_idents = origin_exists_filter_fields_str
            .iter()
            .map(|s| format!("{}_is_exists", s))
            .map(|s| Ident::new(&s, ident.span()))
            .collect::<Vec<_>>();

        let pk_ident = self.pk_ident();
        let pk_type = self.pk_type();
        let pk_str = pk_ident.to_string();
        quote! {
           #(
                pub async fn #fn_idents<'a>(e: impl  ::sqlx::PgExecutor<'a>, #origin_exists_filter_fields: & #origin_exists_filter_types, #pk_ident: ::std::option::Option<#pk_type>)->::sqlx::Result<bool>{
                    let mut q = ::sqlx::QueryBuilder::new(#sql);
                    q.push(format!(" AND {:?}=", #origin_exists_filter_fields_str)).push_bind(#origin_exists_filter_fields);
                    if let Some(v) = #pk_ident {
                        q.push(format!(" AND {:?}<>", #pk_str)).push_bind(v);
                    }
                    let count: (i64,) = q.build_query_as().fetch_one(e).await?;
                    Ok(count.0>0)
                }

           )*

        }
    }
}

mod helper {
    use syn::Ident;

    pub(crate) fn gen_table_name(tn: &str) -> String {
        let mut ss = String::new();

        for (idx, c) in tn.chars().enumerate() {
            if idx == 0 {
                ss.push(c);
                continue;
            }
            if c.is_lowercase() {
                ss.push(c);
                continue;
            }
            if c.is_uppercase() {
                ss.extend(['_', c]);
                continue;
            }
        }

        ss.push('s');

        ss.to_lowercase()
    }

    pub(crate) fn gen_entity_ident(idt: Ident) -> Ident {
        let sa = idt
            .to_string()
            .split("_")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|s| _title(s))
            .collect::<Vec<_>>();
        let ss = sa.join("");

        Ident::new(&ss, idt.span())
    }

    fn _title(s: &str) -> String {
        let mut ss = String::new();
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                ss.push(c.to_ascii_uppercase());
                continue;
            }
            ss.push(c.to_ascii_lowercase())
        }
        ss
    }
}
