use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(EventBridge, attributes(forward_to_trait, trait_returned_error))]
pub fn derive_generate_forward_to(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident;
    let data_enum = match input.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return syn::Error::new_spanned(enum_name, "EventBridge can only be used on enums")
                .to_compile_error()
                .into();
        }
    };

    let trait_name = match get_trait_name(&input.attrs) {
        Ok(trait_name) => trait_name,
        Err(err) => return err.to_compile_error().into(),
    };

    let error_type = match get_error_type(&input.attrs) {
        Ok(error_type) => error_type,
        Err(err) => return err.to_compile_error().into(),
    };

    let match_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let snake_name = to_snake_case(&variant_name.to_string());
        let method_ident = format_ident!("{}", snake_name);

        let (pattern, args) = match &variant.fields {
            Fields::Named(fields) => {
                let field_idents: Vec<Ident> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect();
                let pattern = quote! { #enum_name::#variant_name { #( #field_idents ),* } };
                let args = quote! { #( #field_idents ),* };
                (pattern, args)
            }
            Fields::Unnamed(fields) => {
                let field_idents: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("arg{}", i))
                    .collect();
                let pattern = quote! { #enum_name::#variant_name(#( #field_idents ),*) };
                let args = quote! { #( #field_idents ),* };
                (pattern, args)
            }
            Fields::Unit => {
                let pattern = quote! { #enum_name::#variant_name };
                let args = quote! {};
                (pattern, args)
            }
        };

        quote! {
            #pattern => api.#method_ident(#args).await?,
        }
    });

    let expanded = quote! {
        impl #enum_name {
            pub async fn forward_to<T: #trait_name>(self, api: &mut T) -> ::std::result::Result<(), #error_type> {
                match self {
                    #( #match_arms )*
                }
                Ok(())
            }
        }
    };

    expanded.into()
}

/// Convert CamelCase to snake_case
fn to_snake_case(input: &str) -> String {
    let mut s = String::new();
    for (i, ch) in input.char_indices() {
        if ch.is_uppercase() && i != 0 {
            s.push('_');
        }
        s.push(ch.to_ascii_lowercase());
    }
    s
}

/// Parse the trait name from the #[forward_to_trait(...)] attribute.
fn get_trait_name(attrs: &[Attribute]) -> syn::Result<Ident> {
    for attr in attrs {
        if attr.path().is_ident("forward_to_trait") {
            let path: syn::Path = attr.parse_args()?;
            if let Some(ident) = path.get_ident() {
                return Ok(ident.clone());
            } else {
                return Err(syn::Error::new_spanned(
                    path,
                    "Trait path must be a single identifier",
                ));
            }
        }
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "Missing #[forward_to_trait(TraitName)] attribute",
    ))
}

/// Parse the error type from the #[trait_returned_error(...)] attribute.
fn get_error_type(attrs: &[Attribute]) -> syn::Result<Ident> {
    for attr in attrs {
        if attr.path().is_ident("trait_returned_error") {
            let path: syn::Path = attr.parse_args()?;
            if let Some(ident) = path.get_ident() {
                return Ok(ident.clone());
            } else {
                return Err(syn::Error::new_spanned(
                    path,
                    "Error type path must be a single identifier",
                ));
            }
        }
    }
    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "Missing #[trait_returned_error(ErrorType)] attribute",
    ))
}
