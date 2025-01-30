use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemFn, ItemMod};

/// Custom macro to register all functions in a module as tests.
#[proc_macro_attribute]
pub fn collect_tests(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input item as a module
    let input = parse_macro_input!(item as ItemMod);
    let mod_name = &input.ident;

    // Collect all functions (async or sync) inside the module
    let test_functions: Vec<_> = input
        .content
        .as_ref()
        .unwrap()
        .1
        .iter()
        .filter_map(|item| {
            if let Item::Fn(ItemFn { sig, .. }) = item {
                Some(sig)
            } else {
                None
            }
        })
        .collect();

    // Generate the code to register each test
    let generated_tests = test_functions.iter().map(|sig| {
        let func_name = &sig.ident;
        let is_async = sig.asyncness.is_some(); // Check if the function is async

        if is_async {
            quote! {
                tests.push(libtest_mimic::Trial::test(stringify!(#func_name), || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        #mod_name::#func_name().await
                        }
                    );
                    Ok(())
                }));
            }
        } else {
            quote! {
                tests.push(libtest_mimic::Trial::test(stringify!(#func_name), || {
                    #mod_name::#func_name();
                    Ok(())
                }));
            }
        }
    });

    // Generate the code that will register the tests
    let expanded = quote! {
        #input

        // This code will be injected into the module
        pub fn register_tests(tests: &mut Vec<libtest_mimic::Trial>) {
            #(#generated_tests)*
        }

    };

    // Return the expanded code as TokenStream
    TokenStream::from(expanded)
}
