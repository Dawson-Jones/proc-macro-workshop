use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


fn  inner_ty(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(ref p ) = ty {
        if !(p.path.segments.len() == 1 && p.path.segments[0].ident == "Option") {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref abga) = p.path.segments[0].arguments {
            if abga.args.len() != 1 {
                return None;
            }

            if let syn::GenericArgument::Type(ref t) = abga.args[0] {
                return Some(t);
            }
        }
    }
    None
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());
    let fileds = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed{ref named, ..}),
        ..
    }) = ast.data {
        named
    } else {
        unimplemented!();
    };



    let optionized = fileds.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if inner_ty(ty).is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: std::option::Option<#ty> }
        }
    });

    let methods = fileds.iter().map(|f|{
        let name = &f.ident;
        let ty = &f.ty;
        if let Some(inner_ty) = inner_ty(ty) {
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });
    let build_fileds = fileds.iter().map(|f|{
        let name = &f.ident;
        let ty = &f.ty;
        if inner_ty(ty).is_some() {
            quote! { #name: self.#name.clone() }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });
    let build_empty= fileds.iter().map(|f|{
        let name = &f.ident;
        quote! { #name: None }
    });

    let expanded = quote! {
        pub struct #bident {
            #(#optionized,)*
        }
        impl #bident {
            #(#methods)*

            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#build_fileds,)*
                })
            }
        }

        impl #name {
            fn builder() -> #bident {
                // 这里不能用 Self, Self 是 #name
                #bident {
                    #(#build_empty,)*
                }
            }
        }
    };
    
    expanded.into()
}
