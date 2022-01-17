use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ImplItem, Item, Type};

pub fn parse(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as Item);

    let input = match &input {
        Item::Impl(im) => im,
        _ => return proc_macro::TokenStream::from(quote! {#input}),
    };

    let mut methods = Vec::default();
    let mut params = Vec::default();

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
                            self.service.#name(#(#names),*).await
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

    let service = if let Type::Path(next) = name.as_ref() {
        let ident = next.path.get_ident().expect("ident");
        format_ident!("{}ServiceImpl", ident)
    } else {
        panic!("");
    };

    // let service = format_ident!("{}_Service", name);

    let out = quote! {
        #input

        impl value_invoke::IntoService for #name {
            type Service = #service;
            fn into_service(self) -> Self::Service {

                let params = vec![
                    #(#params),*
                ];

                #service {
                    service: self,
                    params
                }
            }
        }

        pub struct #service {
            service: #name,
            params: Vec<value_invoke::Interface>,
        }

        #[value_invoke::async_trait]
        impl value_invoke::Service for #service {

            fn interface(&self) -> &[value_invoke::Interface] {
                &self.params
            }

            async fn call_method(&self, name: &str, arguments: value_invoke::Arguments) -> Result<value_invoke::value::Value, value_invoke::Error> {

                let interface = match self.params.iter().find(|m| &m.name == name) {
                    Some(p) => p,
                    Nome => {
                        panic!("method not found")
                    }
                };

                interface.parameters.validate(&arguments).unwrap();


                let ret = match name {
                    #(#methods),*,
                    _ => {
                        unreachable!()
                    }

                };

                let value = value::to_value(ret.unwrap()).unwrap();

                Ok(value)
            }
        }
    };

    proc_macro::TokenStream::from(out)
}
