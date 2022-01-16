mod validatable;

#[proc_macro_derive(Validatable)]
pub fn validatable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    validatable::derive(input)
}
