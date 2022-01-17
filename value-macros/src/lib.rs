mod service;
mod validatable;

#[proc_macro_derive(Validatable)]
pub fn validatable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    validatable::derive(input)
}

#[proc_macro_attribute]
pub fn service(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    service::parse(attr, item)
}
