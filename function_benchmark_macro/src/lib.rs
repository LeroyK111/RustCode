/*
开发过程宏\
auto_log过程宏可以作为Rust开发人员的强大工具，它简化了性能基准测试的过程。通过自动化函数执行时间测量，开发人员可以有效地识别瓶颈并优化他们的代码，特别是在CPU或网络密集型操作中。此外，这种方法可以无缝地集成到单元测试中，而不会影响生产代码。
*/

use proc_macro::TokenStream;
use syn::parse_macro_input;

// 当应用这个宏时，它会用时间跟踪包装的函数体，捕捉函数执行的持续时间。
#[proc_macro_attribute]
pub fn auto_log(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    let expanded = quote::quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            println!("Entering function: {}", stringify!(#fn_name));
            #fn_block
            println!("Exiting function: {} (took {} ms)", stringify!(#fn_name), start.elapsed().as_millis());
        }
    };

    TokenStream::from(expanded)
}
