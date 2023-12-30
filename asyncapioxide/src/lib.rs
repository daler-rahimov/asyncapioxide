use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    braced, parse_macro_input, token, AttributeArgs, Expr, Field, ItemEnum, ItemFn, Lit, LitStr,
    Meta, NestedMeta, Result, Token,
};

#[derive(Debug)]
enum Value {
    LitStr(LitStr),
    Expr(Expr),
}

// Define a struct to parse your specific attribute input
#[derive(Default, Debug)]
struct GenDocArgs {
    on: Option<Value>,
    // response_model: Option<String>,
    // request_model: String,
    // emits: Vec<EmitsArg>,
}
impl Parse for GenDocArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const EXPECTED_ATTRIBUTE_MESSAGE: &str =
            "unexpected identifier, expected any of: on, response_model, request_model, emits";
        let mut path_attr = GenDocArgs::default();

        println!("input: {:?}", input);

        while !input.is_empty() {
            let ident = input.parse::<Ident>().map_err(|error| {
                syn::Error::new(
                    error.span(),
                    format!("{EXPECTED_ATTRIBUTE_MESSAGE}, {error}"),
                )
            })?;
            let attribute_name = &*ident.to_string();
            match attribute_name {
                "on" => {
                    path_attr.on = Some(Value::LitStr(input.parse::<LitStr>()?));
                }
                _ => {
                    return Err(syn::Error::new(ident.span(), EXPECTED_ATTRIBUTE_MESSAGE));
                }
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(path_attr)
    }
}

// #[derive(Debug)]
// struct EmitsArg {
//     event: String,
//     model: String,
//     description: String,
// }

#[proc_macro_attribute]
pub fn gen_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function this attribute is attached to
    let input_fn = parse_macro_input!(item as ItemFn);

    // Parse the attributes passed to the macro
    let args = parse_macro_input!(attr as GenDocArgs);

    // Generate or modify code based on parsed attributes
    let expanded = quote! {
        #input_fn
    };

    TokenStream::from(expanded)
}

// fn process_args(args: AttributeArgs) {
//     // Parse each argument
//     for arg in args {
//         println!("arg: {:?}", arg);
//     }
// }
