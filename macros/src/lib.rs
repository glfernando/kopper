
extern crate proc_macro;

use quote::quote;
use syn::{ItemFn, parse_macro_input};
use proc_macro::{TokenStream, TokenTree::{Ident, Literal}};


#[proc_macro_attribute]
pub fn shell_cmd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);

    // TODO: create parser for attr
    let mut attr_it = attr.into_iter();
    let cmd_name = match attr_it.next().unwrap() {
        Ident(ident) => ident,
        _ => panic!("expected ident for command name")
    };

    attr_it.next(); // skip coma puntuation

    let help_lit = match attr_it.next().unwrap() {
        Literal(literal) => literal,
        _ => panic!("expected literal for command help")
    };

    let section = format!(".shell_cmds.{}", cmd_name);
    let cmd_name_str = format!("{}", cmd_name);
    let help_str = help_lit.to_string().trim_matches('"').to_string();
    let func_name = &func.sig.ident;
    let var_name = format!("{}", func_name).to_uppercase();
    let var_name = syn::Ident::new(&var_name, func_name.span());
    quote!(
        #[doc(hidden)]
        #func

        #[link_section = #section]
        #[used]
        static #var_name: ConCmd = ConCmd::new(#cmd_name_str, #help_str, #func_name);
    )
    .into()
}
