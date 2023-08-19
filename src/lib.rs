use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument,
    Generics, Ident, PathArguments, Type,
};

#[proc_macro_derive(Builder)]
pub fn derive_builder(annotated_item: TokenStream) -> TokenStream {
    let DeriveInput {
        data,
        generics,
        ident,
        ..
    } = parse_macro_input!(annotated_item);

    match data {
        Data::Struct(struct_body) => struct_builder(struct_body, generics, ident),
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}

fn struct_builder(body: DataStruct, generics: Generics, struct_identifier: Ident) -> TokenStream {
    let builder_identifier = builder_identifier(&struct_identifier);

    // Attach builder function to annotated item
    let mut builder: TokenStream = quote! {
        impl #generics #struct_identifier #generics {
            pub fn builder() -> #builder_identifier #generics {
                #builder_identifier::new()
            }
        }
    }
    .into();

    // Attach a builder struct fields and implement
    let builder_implementation = match body.fields {
        Fields::Named(fields) => {
            struct_with_named_fields_tokens(fields, generics, struct_identifier)
        }
        Fields::Unnamed(_) => unimplemented!(),
        Fields::Unit => unimplemented!(),
    };

    builder.extend(builder_implementation);
    builder
}

fn struct_with_named_fields_tokens(
    fields: FieldsNamed,
    generics: Generics,
    struct_identifier: Ident,
) -> TokenStream {
    let builder_identifier = builder_identifier(&struct_identifier);

    let mut declarations = vec![];
    let mut initializers = vec![];
    let mut setters = vec![];
    let mut getters = vec![];

    for field in fields.named.iter() {
        let (declare, initialize, set, get) = named_fields_tokens(field);
        declarations.push(declare);
        initializers.push(initialize);
        setters.push(set);
        getters.push(get)
    }

    quote! {
        struct #builder_identifier #generics {
            #(#declarations)*
        }

        impl #generics #builder_identifier #generics {
            pub fn new() -> Self  {
                #builder_identifier {
                    #(#initializers)*
                }
            }

            pub fn build(&self) -> Result<#struct_identifier #generics, Box<dyn std::error::Error>> {
                let result = #struct_identifier {
                    #(#getters)*
                };

                Ok(result)
            }

            #(#setters)*
        }
    }
    .into()
}

fn named_fields_tokens(field: &Field) -> (TokenStream2, TokenStream2, TokenStream2, TokenStream2) {
    let Field {
        ident: field_name,
        ty: field_type,
        ..
    } = field;
    match match_field_type(field_type) {
        MatchedFieldType::Option(t) => (
            quote! {
                #field_name: #field_type,
            },
            quote! {
                #field_name: None,
            },
            quote! {
                pub fn #field_name(mut self, #field_name: #t) -> Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            },
            quote! {
                #field_name: self.#field_name.to_owned(),
            },
        ),
        MatchedFieldType::Vec(t) => (
            quote! {
                #field_name: #field_type,
            },
            quote! {
                #field_name: vec![],
            },
            quote! {
                pub fn #field_name(mut self, #field_name: #t) -> Self {
                    self.#field_name.push(#field_name);
                    self
                }
            },
            quote! {
                #field_name: self.#field_name.to_owned(),
            },
        ),
        MatchedFieldType::Other => (
            quote! {
                #field_name: Option<#field_type>,
            },
            quote! {
                #field_name: None,
            },
            quote! {
                pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name = Some(#field_name);
                    self
                }
            },
            quote! {
                #field_name: self.#field_name.to_owned().ok_or("Field #field_name unset")?,
            },
        ),
    }
}

fn builder_identifier(identifier: &Ident) -> Ident {
    Ident::new(&format!("{}Builder", identifier), Span::call_site())
}

// TODO: This is too messy, find a better approach to parse declarations.
fn match_field_type(field_type: &Type) -> MatchedFieldType {
    let path_segments = match field_type {
        Type::Path(path) if path.qself.is_none() => &path.path.segments,
        _ => return MatchedFieldType::Other,
    };
    let last_segment = path_segments.iter().last().unwrap();
    let segment_arguments = match &last_segment.arguments {
        PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
        _ => return MatchedFieldType::Other,
    };
    match segment_arguments {
        GenericArgument::Type(t) if last_segment.ident == "Option" => MatchedFieldType::Option(t),
        GenericArgument::Type(t) if last_segment.ident == "Vec" => MatchedFieldType::Vec(t),
        _ => MatchedFieldType::Other,
    }
}

enum MatchedFieldType<'t> {
    Vec(&'t Type),
    Option(&'t Type),
    Other,
}
