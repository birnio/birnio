use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Derives constructor methods for Route enum variants
///
/// # Example
///
/// ```ignore
/// #[derive(RouteConstructors)]
/// pub enum Route {
///     Home(home::State),
///     Settings(settings::State),
///     Workspaces,
/// }
/// ```
///
/// Generates:
///
/// ```ignore
/// impl Route {
///     pub fn home() -> Self {
///         Route::Home(home::State::default())
///     }
///
///     pub fn settings() -> Self {
///         Route::Settings(settings::State::default())
///     }
///
///     pub fn workspaces() -> Self {
///         Route::Workspaces
///     }
/// }
/// ```
#[proc_macro_derive(RouteConstructors)]
pub fn route_constructors_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("RouteConstructors can only be derived for enums"),
    };

    let constructors = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let fn_name = snake_case(&variant_name.to_string());
        let fn_name_ident = syn::Ident::new(&fn_name, variant_name.span());

        let doc = format!("Create the {} route", variant_name);

        match &variant.fields {
            // Variant with a single field: Home(home::State)
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field_type = &fields.unnamed.first().unwrap().ty;
                quote! {
                    #[doc = #doc]
                    pub fn #fn_name_ident() -> Self {
                        #enum_name::#variant_name(<#field_type>::default())
                    }
                }
            }
            // Unit variant: Workspaces
            Fields::Unit => {
                quote! {
                    #[doc = #doc]
                    pub fn #fn_name_ident() -> Self {
                        #enum_name::#variant_name
                    }
                }
            }
            _ => panic!("RouteConstructors only supports unit variants or variants with a single unnamed field"),
        }
    });

    let expanded = quote! {
        impl #enum_name {
            #(#constructors)*
        }
    };

    TokenStream::from(expanded)
}

fn snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }

    result
}
