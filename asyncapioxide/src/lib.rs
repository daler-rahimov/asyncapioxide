use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, LitStr, Token};

struct EmitArg {
    event: String,
    model: String,
    description: String,
}

struct GenDocArgs {
    on: Option<String>,
    response_model: Option<String>,
    request_model: Option<String>,
    emits: Vec<EmitArg>,
}

impl syn::parse::Parse for GenDocArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut on = None;
        let mut response_model = None;
        let mut request_model = None;
        let mut emits = Vec::new();

        while !input.is_empty() {
            let lookahead = input.lookahead1();

            if lookahead.peek(Ident) && input.peek2(Token![=]) {
                let ident: Ident = input.parse()?;
                let _equals: Token![=] = input.parse()?;

                match ident.to_string().as_str() {
                    "on" => {
                        if on.is_some() {
                            return Err(input.error("duplicate 'on' definition"));
                        }
                        on = Some(input.parse::<LitStr>()?.value());
                    }
                    "response_model" => {
                        if response_model.is_some() {
                            return Err(input.error("duplicate 'response_model' definition"));
                        }
                        response_model = Some(input.parse::<LitStr>()?.value());
                    }
                    "request_model" => {
                        if request_model.is_some() {
                            return Err(input.error("duplicate 'request_model' definition"));
                        }
                        request_model = Some(input.parse::<LitStr>()?.value());
                    }
                    // add parsing logic for emits here
                    "emits" => {
                        if !emits.is_empty() {
                            return Err(input.error("duplicate 'emits' definition"));
                        }

                        let content;
                        syn::bracketed!(content in input);
                        while !content.is_empty() {
                            let content_lookahead = content.lookahead1();
                            if content_lookahead.peek(syn::token::Paren) {
                                let emit_content;
                                syn::parenthesized!(emit_content in content);
                                let mut event = None;
                                let mut model = None;
                                let mut description = None;

                                while !emit_content.is_empty() {
                                    let emit_lookahead = emit_content.lookahead1();

                                    if emit_lookahead.peek(Ident) && emit_content.peek2(Token![=]) {
                                        let ident: Ident = emit_content.parse()?;
                                        let _equals: Token![=] = emit_content.parse()?;

                                        match ident.to_string().as_str() {
                                            "event" => {
                                                if event.is_some() {
                                                    return Err(emit_content
                                                        .error("duplicate 'event' definition"));
                                                }
                                                event =
                                                    Some(emit_content.parse::<LitStr>()?.value());
                                            }
                                            "model" => {
                                                if model.is_some() {
                                                    return Err(emit_content
                                                        .error("duplicate 'model' definition"));
                                                }
                                                model =
                                                    Some(emit_content.parse::<LitStr>()?.value());
                                            }
                                            "description" => {
                                                if description.is_some() {
                                                    return Err(emit_content.error(
                                                        "duplicate 'description' definition",
                                                    ));
                                                }
                                                description =
                                                    Some(emit_content.parse::<LitStr>()?.value());
                                            }
                                            _ => {
                                                return Err(emit_content
                                                    .error("unexpected attribute in emits"))
                                            }
                                        }
                                    } else if emit_lookahead.peek(Token![,]) {
                                        let _: Token![,] = emit_content.parse()?;
                                    } else {
                                        return Err(emit_lookahead.error());
                                    }
                                }

                                emits.push(EmitArg {
                                    event: event.ok_or_else(|| {
                                        emit_content.error("missing 'event' in emit")
                                    })?,
                                    model: model.ok_or_else(|| {
                                        emit_content.error("missing 'model' in emit")
                                    })?,
                                    description: description.ok_or_else(|| {
                                        emit_content.error("missing 'description' in emit")
                                    })?,
                                });
                            } else {
                                return Err(content_lookahead.error());
                            }
                        }
                    }
                    _ => return Err(input.error("unexpected attribute")),
                }
            } else if lookahead.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(GenDocArgs {
            on,
            response_model,
            request_model,
            emits,
        })
    }
}

#[proc_macro_attribute]
pub fn asyncapi_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the original function this attribute is attached to
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Parse the attributes passed to the macro
    let args = parse_macro_input!(attr as GenDocArgs);

    let response_model = args.response_model.unwrap(); // handle errors/none in real use
    let request_model = args.request_model.unwrap(); // handle errors/none in real use

    // Convert the type names to Ident
    let response_model_ident = Ident::new(&response_model, proc_macro2::Span::call_site());
    let request_model_ident = Ident::new(&request_model, proc_macro2::Span::call_site());
    let on = args.on.unwrap();
    let emits = args.emits;

    // Generate print statements for each emit
    let emit_prints = emits.iter().map(|emit| {
        let event = &emit.event;
        let model = &emit.model;
        let description = &emit.description;
        quote! {
            println!("emit event: {}, model: {}, description: {}", #event, #model, #description);
        }
    });

    let schema_print_statements = quote! {
        fn gen_asyncapi_docs() {
            let schema1 = schemars::schema_for!(#request_model_ident);
            let schema2 = schemars::schema_for!(#response_model_ident);
            println!("request schema1: {}", serde_json::to_string_pretty(&schema1).unwrap());
            println!("response schema2: {}", serde_json::to_string_pretty(&schema2).unwrap());
            println!("on: {}", #on);
            #(#emit_prints)*
        }
    };

    // Modify the original function to call main2
    let original_block = input_fn.block;
    input_fn.block = syn::parse(
        quote! {
            {
                gen_asyncapi_docs(); // call the generated main2 function
                #original_block // original function body
            }
        }
        .into(),
    )
    .unwrap();

    // Combine the generated function and the modified original function
    let expanded = quote! {
        #schema_print_statements
        #input_fn
    };

    TokenStream::from(expanded)
}
