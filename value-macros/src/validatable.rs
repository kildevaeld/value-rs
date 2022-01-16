use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index, Type,
};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let validation = create_validation(&input.data);

    let output = quote! {
        impl #impl_generics value_validate::Validatable for #name #ty_generics #where_clause {
            fn validator() -> value_validate::Validator {
                #validation
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(value_validate::Validatable));
        }
    }
    generics
}

fn create_validation(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            o = o.field(stringify!(#name), <#ty as value_validate::Validatable>::validator());
                        }
                    });
                    quote! {
                        let mut o = value_validate::object();
                        #(
                            #recurse
                        )*
                        o.into()
                    }
                }
                Fields::Unnamed(ref fields) => {
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        // let index = Index::from(i);
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            <#ty as value_validate::Validatable>::validator().boxed()
                        }
                    });
                    quote! {
                        use value_validate::{ValidationExt, ValidatorBuilderExt};
                        value_validate::list().and(value_validate::tuple(vec![
                            #(#recurse),*
                        ])).into()
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote! {
                        let mut o = value_validate::any();
                        o.into()
                    }
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
