extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use syn::{AttributeArgs};

use syn::Lit::Str;

#[proc_macro_attribute]
pub fn main(input_args: TokenStream, _: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(input_args as AttributeArgs);

    build_bevy(args)
}

fn build_bevy(args: syn::AttributeArgs) -> TokenStream {
    let mut system_tokens = quote! {};
    let mut plugin_tokens = quote!{};

    for arg in args {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(namevalue)) => {
                let ident = namevalue.path.get_ident().unwrap();

                match ident.to_string().as_str() {
                    "systems" => {
                        let lit = namevalue.lit.clone();

                        match lit {
                            Str(v) => {
                                let v = v.value();

                                let systems: Vec<TokenStream2> = v
                                    .split(",")
                                    .map(|f| {
                                        let system_ident = Ident::new(f, Span::call_site());

                                        quote! {
                                            #system_ident
                                        }
                                    })
                                    .collect();

                                system_tokens = quote! {
                                    #(.add_system(#systems.system()))*
                                };
                            }
                            _ => {}
                        }
                    }
                    "plugins" => {
                        let lit = namevalue.lit.clone();

                        match lit {
                            Str(v) => {
                                let v = v.value();

                                let plugins: Vec<TokenStream2> = v
                                    .split(",")
                                    .map(|f| {
                                        let plugin_ident = Ident::new(f, Span::call_site());

                                        quote! {
                                            #plugin_ident
                                        }
                                    })
                                    .collect();

                                    plugin_tokens = quote! {
                                    #(.add_plugin(#plugins))*
                                };
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            // syn::NestedMeta::Meta(syn::Meta)
            _ => {
                // syn::Meta::Path()
                // println!("{:?}",_)
            }
        }
    }

    let t = quote! {
        fn main() {
            App::build()
            // .add_plugins(DefaultPlugins)
            #system_tokens
            #plugin_tokens
            .run();
        }
    };

    t.into()
}
