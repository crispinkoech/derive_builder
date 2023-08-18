use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Generics, Ident,
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

    let mut builder: TokenStream = quote! {
        impl #generics #struct_identifier #generics {
            pub fn builder() -> #builder_identifier #generics {
                #builder_identifier::new()
            }
        }
    }
    .into();

    let builder_implementation = match body.fields {
        Fields::Named(fields) => {
            struct_with_named_fields_builder(fields, generics, struct_identifier)
        }
        Fields::Unnamed(_) => unimplemented!(),
        Fields::Unit => unimplemented!(),
    };

    builder.extend(builder_implementation);
    builder
}

fn struct_with_named_fields_builder(
    fields: FieldsNamed,
    generics: Generics,
    struct_identifier: Ident,
) -> TokenStream {
    let builder_identifier = builder_identifier(&struct_identifier);

    let mut types = vec![];
    let mut initial_fields = vec![];
    let mut setters = vec![];

    for field in fields.named.iter() {
        let Field {
            ident: field_name,
            ty: field_type,
            ..
        } = field;

        types.push(quote! {
            #field_name: Option<#field_type>,
        });

        initial_fields.push(quote! {
            #field_name: None,
        });

        setters.push(quote! {
            pub fn #field_name(mut self, #field_name: #field_type) -> Self {
                self.#field_name = Some(#field_name);
                self
            }
        })
    }

    quote! {
        struct #builder_identifier #generics {
            #(#types)*
        }

        impl #generics #builder_identifier #generics {
            pub fn new () -> Self  {
                #builder_identifier {
                    #(#initial_fields)*
                }
            }

            #(#setters)*
        }
    }
    .into()
}

fn builder_identifier(identifier: &Ident) -> Ident {
    Ident::new(&format!("{}Builder", identifier), Span::call_site())
}
