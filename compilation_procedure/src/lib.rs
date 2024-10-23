use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Lit};
#[proc_macro_derive(NoOp)]
pub fn no_op_derive(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
#[proc_macro_derive(ToDatabaseQuery, attributes(field_type))]
pub fn derive_to_document(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut fields_code = quote! {};
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                let field_name = &field.ident;
                let mut final_field_name = quote! { stringify!(#field_name) }; // Default to the field name
                let mut field_type = None;
                // Combine attribute checks into a single iteration
                for attr in &field.attrs {
                    if attr.path().is_ident("serde") {
                        let _ = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("rename") {
                                let value = meta.value()?;
                                let rename_value: Lit = value.parse()?;
                                if let Lit::Str(rename_str) = rename_value {
                                    final_field_name = quote! { #rename_str };
                                }
                            }
                            Ok(())
                        });
                    }
                    if attr.path().is_ident("field_type") {
                        let _ = attr.parse_nested_meta(|meta| {
                            field_type = Some(meta.path);
                            Ok(())
                        });
                    }
                }
                let field_ty = &field.ty; // Obtiene el tipo del campo
                let insert_code = match field_type {
                    Some(path) if path.is_ident("mandatory") => {
                        // Campo obligatorio
                        quote! {
                            doc.insert(#final_field_name, self.#field_name.clone());
                        }
                    }
                    Some(path) if path.is_ident("optional_with_none") => {
                        // Campo opcional que agrega `None` explícitamente
                        quote! {
                            doc.insert(#final_field_name, self.#field_name.clone().unwrap_or_else(|| None::<#field_ty>));
                        }
                    }
                    Some(path) if path.is_ident("optional_omit_none") => {
                        // Campo opcional que se omite si es `None`
                        quote! {
                            if let Some(value) = &self.#field_name {
                                doc.insert(#final_field_name, value.clone());
                            }
                        }
                    }
                    None => {
                        // Default to `mandatory`
                        quote! {
                            doc.insert(#final_field_name, self.#field_name.clone());
                        }
                    }
                    _ => quote! {},
                };
                fields_code = quote! {
                    #fields_code
                    #insert_code
                }
            }
        }
    }

    let expanded = quote! {
        impl ToDocument for #name {
            fn to_doc(&self) -> Result<bson::Document, bson::ser::Error> {
                let mut doc = bson::Document::new();
                #fields_code
                Ok(doc)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToInsert)]
pub fn to_insert_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident; // Nombre original de la estructura
    let new_struct_name = Ident::new(
        &format!("{}Attributes", struct_name),
        proc_macro::Span::call_site().into(),
    ); // Nuevo nombre con sufijo Attributes

    // Obtenemos los campos de la estructura original y los filtramos
    let fields = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => {
                fields_named
                    .named
                    .iter()
                    .filter_map(|field| {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        // Filtrar campos no deseados
                        if field_name == "created_at" || field_name == "updated_at" {
                            return None; // No incluimos estos campos
                        }
                        let serde_attrs = field
                            .attrs
                            .iter()
                            .filter(|attr| attr.path().is_ident("serde"))
                            .collect::<Vec<_>>();
                        // Usamos el campo original sin atributos
                        let field_ident = format_ident!("{}", field_name);
                        let field_ty = &field.ty;
                        Some(quote! {
                            #(#serde_attrs)*
                            pub #field_ident: #field_ty,
                        })
                    })
                    .collect::<Vec<_>>()
            }
            _ => panic!("ToInsert solo soporta estructuras con campos nombrados"),
        },
        _ => panic!("ToInsert solo soporta estructuras"),
    };

    // Generamos el código para la nueva estructura y su implementación
    let gen = quote! {
        #[derive(Serialize, Deserialize, Clone, Debug, Builder)]
        pub struct #new_struct_name {
            #(#fields)* // Usamos los campos filtrados sin atributos
        }

        impl ToDocument for #new_struct_name {
            fn to_doc(&self) -> Result<bson::Document, bson::ser::Error> {
                let doc = bson::to_bson(self)?;
        let document = doc.as_document().ok_or_else(||{
            Error::InvalidCString("Error converting to document".to_string())
        })?;
        Ok(document.to_owned())
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(WithId)]
pub fn as_with_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let new_struct_name = Ident::new(
        &format!("{}WithId", struct_name),
        proc_macro::Span::call_site().into(),
    );

    // Verificar si el campo `id` ya está definido en la estructura principal
    let mut has_id_field = false;
    let fields = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields_named) => fields_named
                .named
                .iter()
                .map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    if field_ident == "id" {
                        has_id_field = true;
                    }
                    let field_ty = &field.ty;
                    let attrs = &field.attrs;
                    quote! {
                        #(#attrs)*
                        pub #field_ident: #field_ty,
                    }
                })
                .collect::<Vec<_>>(),
            _ => panic!("WithId solo soporta estructuras con campos nombrados"),
        },
        _ => panic!("WithId solo soporta estructuras"),
    };

    // Generar la nueva estructura
    let gen = if has_id_field {
        quote! {
            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub struct #new_struct_name {
                #(#fields)*
            }
        }
    } else {
        quote! {
            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub struct #new_struct_name {
                #[serde(rename = "_id")]
                pub id: ObjectId,
                #(#fields)*
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Database, attributes(database, index))]
pub fn derive_database(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let base_struct_name = input.ident;
    let struct_name = Ident::new(
        format!("{}Schema", base_struct_name).as_str(),
        proc_macro::Span::call_site().into(),
    );

    let mut database_name = None;
    let mut collection_name = None;
    let mut indices = Vec::new();

    for attr in input.attrs {
        if attr.path().is_ident("database") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("database") {
                    let value = meta.value()?;
                    let lit_str: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit_str {
                        database_name = Some(lit_str.value());
                    }
                }
                if meta.path.is_ident("collection") {
                    let value = meta.value()?;
                    let lit_str: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit_str {
                        collection_name = Some(lit_str.value());
                    }
                }
                Ok(())
            });
        }
        if attr.path().is_ident("index") {
            let mut keys = Vec::new();
            let mut unique = false;
            let mut name = None;

            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("keys") {
                    let value = meta.value()?;
                    let lit_str: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit_str {
                        keys = lit_str
                            .value()
                            .split(',')
                              .map(|s| {
                                let mut parts = s.split(':');
                                let key = parts.next().unwrap().trim().to_string();
                                let value:String = parts.next().unwrap().trim().parse().unwrap();
                                (key, value)
                            })
                            .collect();
                    }
                } if meta.path.is_ident("unique") {
                    let value = meta.value()?;
                    let lit_bool: Lit = value.parse()?;
                    if let Lit::Bool(lit_bool) = lit_bool {
                        unique = lit_bool.value;
                    }
                }if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let lit_str: Lit = value.parse()?;
                    if let Lit::Str(lit_str) = lit_str {
                        name = Some(lit_str.value());
                    }
                }
                Ok(())
            });

            indices.push((keys, unique, name));
        }
    }

    let database_name = database_name.expect("database attribute is required");
    let collection_name = collection_name.expect("collection attribute is required");
    let index_models = indices.iter().map(|(keys, unique, name)| {
        let keys_doc = keys.iter().map(|(key, value)| {
            quote! { #key: #value }
        });
        let name = name.clone().unwrap_or_else(|| "index".to_string());
        quote! {
            indexes.push(IndexModel::builder()
                .keys(doc! { #(#keys_doc),* })
                .options(
                    IndexOptions::builder()
                        .unique(#unique)
                        .name(#name.to_string())
                        .build(),
                )
                .build());
        }
    });
    let expanded = quote! {
        pub struct #struct_name;
        impl BaseColleccionNames for #base_struct_name {
            fn get_collection_name() -> &'static str {
                #collection_name
            }

            fn get_database_name() -> &'static str {
                #database_name
            }
        }
        #[async_trait]
        impl Schema for #struct_name{
            fn get_collection_name(&self) -> &'static str {
                #base_struct_name::get_collection_name()
            }

            fn get_database_name(&self) -> &'static str {
                #base_struct_name::get_database_name()
            }

            async fn set_indexes(
                &self,
                client: &Client,
            ) -> Result<Option<CreateIndexesResult>, mongodb::error::Error> {
                let collection = client
                    .database(self.get_database_name())
                    .collection::<#base_struct_name>(self.get_collection_name());
                let mut indexes: Vec<IndexModel> = vec![];
                #(#index_models)*
                let _ = IndexFunctions::delete_existing_indexes(&collection, &mut indexes).await;
                let option: Option<CreateIndexesResult> = None;
                if indexes.is_empty() {
                    return Ok(option);
                }
                Ok(Some(collection.create_indexes(indexes).await?))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToBson)]
pub fn to_bson_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl From<#name> for Bson {
            fn from(item: #name) -> Bson {
                to_bson(&item).unwrap_or(Bson::Null)
            }
        }
    };

    TokenStream::from(expanded)
}