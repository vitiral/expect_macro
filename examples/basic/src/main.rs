
#[macro_use]
extern crate expect_macro;

fn main() {
    let y = expect!(Some(3));
    let x = expect!(Some(4), "need 4");

    println!("{}, {}", x, y);
}
