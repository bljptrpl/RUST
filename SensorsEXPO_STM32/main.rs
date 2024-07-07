// fn hello(val:bool) -> &str{
//     if val{
//         "Hello Rustacean !"
//     }
//     else {
//         "Hello there !"
//     }
// }
// fn main(){
//     let bool_val:bool = false;
//     println!("{}", hello(bool_val));
// }


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
