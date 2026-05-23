use proc_macro::TokenStream;
use quote::{ToTokens, quote};

pub fn parser_logger(func: &syn::ImplItemFn) -> TokenStream {

    let name   = &func.sig.ident;
    let inputs = &func.sig.inputs;
    let output = &func.sig.output;
    let body   = &func.block;

    let name_str = name.to_token_stream().to_string();

    quote! {
        fn #name(#inputs) #output {

            if !DEBUG_LOG {
                return {
                    #body
                }
            }

            let name  = #name_str;
            let token = self.peek();

            let depth = self.logger.depth.get();
            let ind   = "| ".repeat(depth);

            println!("{ind}{name} ({token}) {{");

            self.logger.enter();

            // closure to catch `?` early returns
            let res = (|| {
                #body
            })();

            self.logger.exit(depth);

            match &res {
                Err(err) => println!("{ind}}} /{name} (ERR: ({:?}))", err),
                Ok (_)   => println!("{ind}}} /{name}"),
            };

            res
        }
    }.into()
}
