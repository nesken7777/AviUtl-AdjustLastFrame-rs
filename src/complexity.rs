use complexity::Complexity;
use syn::{parse_quote, ItemFn};

fn main() {
    let example: ItemFn = parse_quote! {
            fn examplefn()->i32{
            1
        }
    };
    println!("complexity:{}",example.complexity());
}
