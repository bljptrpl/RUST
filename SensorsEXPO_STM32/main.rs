// Lab 1 - 
fn hello<'a>(val: bool) -> &'a str {
    if val == true{
        "Hello, Rustacean!"
    } else {
        "Hello there!"
    }
}

fn main() {
    let bool_val: bool = true;
    println!("{}", hello(bool_val));
}
