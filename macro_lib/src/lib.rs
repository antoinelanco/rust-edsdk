use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(IntoI32)]
pub fn derive_into_i32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &input.data {
        variants
    } else {
        return syn::Error::new_spanned(input.ident, "IntoI32 can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut arms = vec![];

    for variant in variants {
        let ident = &variant.ident;

        let value = if let Some((_, expr)) = &variant.discriminant {
            expr.clone()
        } else {
            return syn::Error::new_spanned(
                variant.ident.clone(),
                "Each enum variant must have an explicit discriminant value (e.g. Foo = 1)",
            )
            .to_compile_error()
            .into();
        };

        arms.push(quote! {
            #enum_name::#ident => #value,
        });
    }

    let expanded = quote! {
        impl ::core::convert::Into<i32> for #enum_name {
            fn into(self) -> i32 {
                match self {
                    #(#arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TryFromI32)]
pub fn derive_try_from_i32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &input.data {
        variants
    } else {
        return syn::Error::new_spanned(enum_name, "TryFromI32 can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut try_from_arms = vec![];

    for variant in variants {
        let ident = &variant.ident;

        let value = if let Some((_, expr)) = &variant.discriminant {
            expr.clone()
        } else {
            return syn::Error::new_spanned(
                variant.ident.clone(),
                "Each enum variant must have an explicit discriminant value (e.g. Foo = 1)",
            )
            .to_compile_error()
            .into();
        };

        try_from_arms.push(quote! {
            #value => Ok(#enum_name::#ident),
        });
    }

    let expanded = quote! {
        impl ::core::convert::TryFrom<i32> for #enum_name {
            type Error = i32;

            fn try_from(i: i32) -> Result<Self, Self::Error> {
            let u = match u32::try_from(i) {
                Ok(v) => v,
                Err(_) => return Err(i),
            };
            match u {
                #(#try_from_arms)*
                _ => Err(i),
            }
        }
        }
    };
    TokenStream::from(expanded)
}
