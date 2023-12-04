use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // origin
    // let _ = input;
    // unimplemented!()

    // 01-parse.rs
    let ast = parse_macro_input!(input as DeriveInput);
    // let ast: DeriveInput = syn::parse(input).unwrap();
    // eprintln!("{:#?}", ast);
    // TokenStream::new()

    // 02-create-builder.rs
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());
    let expanded = quote! {
        struct #bident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }
        impl #name {
            fn builder() -> #bident {
                // 这里不能用 Self, Self 是 #name
                #bident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };
    
    expanded.into()
}
