extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::AttributeArgs;

use syn::Lit::Str;

#[proc_macro_attribute]
pub fn main(input_args: TokenStream, input_item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input_args as AttributeArgs);

    let item = syn::parse_macro_input!(input_item as syn::ItemFn);

    let body = &item.block.stmts;

    build_bevy(args, body)
}

fn build_bevy(args: syn::AttributeArgs, body: &Vec<syn::Stmt>) -> TokenStream {

    let mut fn_tokens:Vec<TokenStream2> = Vec::new();

    for (i, arg) in args.iter().enumerate() {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(namevalue)) => {
                let ident = namevalue.path.get_ident().unwrap().to_string();

                let lit = namevalue.lit.clone();

                match lit {
                    Str(v) => {
                        let v = v.value();

                        let tokens: Vec<TokenStream2> = v
                            .split(",")
                            .map(|f| {
                                let ident_token = Ident::new(f, Span::call_site());

                                quote! {
                                    #ident_token
                                }
                            })
                            .collect();

                        if ident.contains("system") {
                            let mut fn_name = String::from("add_");
                            fn_name.push_str(&ident);
                            let ident_fn = Ident::new(&fn_name,Span::call_site());
                            fn_tokens.push( quote! {
                                #(.#ident_fn(#tokens.system()))*
                            });
                        } else {
                            let mut fn_name = String::from("add_");
                            fn_name.push_str(&ident);
                            let ident_fn = Ident::new(&fn_name,Span::call_site());
                            fn_tokens.push( quote! {
                                #(.#ident_fn(#tokens))*
                            });
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let t = quote! {
        fn main() {
            #(#body)*

            App::build()
            .add_plugins(DefaultPlugins)
            // #system_tokens
            // #plugin_tokens
            #(#fn_tokens)*
            .run();
        }
    };

    t.into()
}
