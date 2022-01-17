use quote::quote;
use syn::{parse_macro_input, FnArg, ImplItem, Item, ItemImpl};
pub fn parse(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as Item);

    let input = match &input {
        Item::Impl(im) => im,
        _ => return proc_macro::TokenStream::from(quote! {#input}),
    };

    let mut methods = Vec::default();
    let mut params = Vec::default();
    // let mut cmds = Vec::default();

    for item in &input.items {
        let item =
            match item {
                ImplItem::Method(method) => {
                    let name = &method.sig.ident;

                    if method.sig.asyncness.is_none() {
                        // method.attrs.iter().find(|m| m.)
                        continue;
                    }

                    let args =
                        method.sig.inputs.iter().enumerate().filter_map(
                            |(idx, input)| match input {
                                FnArg::Receiver(_) => None,
                                FnArg::Typed(arg) => {
                                    //
                                    let name = &arg.pat;
                                    let ty = &arg.ty;
                                    let idx = idx - 1;
                                    Some(quote! {
                                        let #name: #ty = arguments.try_get(#idx).unwrap();
                                    })
                                }
                            },
                        );

                    let names = method.sig.inputs.iter().filter_map(|input| match input {
                        FnArg::Receiver(_) => None,
                        FnArg::Typed(arg) => {
                            let name = &arg.pat;
                            Some(quote! {
                                #name
                            })
                        }
                    });

                    let types = method.sig.inputs.iter().filter_map(|input| match input {
                        FnArg::Receiver(_) => None,
                        FnArg::Typed(arg) => {
                            let ty = &arg.ty;
                            Some(quote! {
                                <#ty as value_validate::Validatable>::validator()
                            })
                        }
                    });

                    params.push(quote! {
                        {
                            let mut params = value_invoke::Parameters::default();

                            #(
                                params = params.add(value_invoke::Parameter::new(#types));
                            )*

                            value_invoke::Interface {
                                name: stringify!(#name).to_owned(),
                                parameters: params
                            }
                        }
                    });

                    quote! {
                       stringify!(#name) => {
                            #( #args )*;
                            self.#name(#(#names),*).await
                       }
                    }

                    // if method.sig.asyncness.is_some() {
                    //     quote! {

                    //     };
                    // }
                }
                _ => {
                    //
                    continue;
                }
            };

        methods.push(item);
    }

    let name = &input.self_ty;

    let out = quote! {
        #input

        impl #name {

            pub fn interface(&self) -> Vec<value_invoke::Interface> {
                vec![
                    #(#params),*
                ]
            }

            pub async fn call<A: value_invoke::IntoArguments>(&self, name: &str, arguments: A) -> Result<value::Value, value_invoke::Error> {
                //
                let arguments = arguments.into_arguments().unwrap();

                let ret = match name {
                    #(#methods),*,
                    _ => {
                        panic!("invalid method");
                    }

                };

                let value = value::to_value(ret.unwrap()).unwrap();

                Ok(value)
            }
        }
    };

    proc_macro::TokenStream::from(out)
}
