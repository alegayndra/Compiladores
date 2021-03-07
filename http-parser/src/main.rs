// code taken from https://github.com/zupzup/rust-nom-parsing

use http_parser::uri;

fn main() {
    let res = uri("https://zupzup.org/about");
    println!("Result: {:?}", res);

    let res = uri("https:invalid");
    println!("Result: {:?}", res);
}