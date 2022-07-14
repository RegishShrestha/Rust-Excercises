
//4. Make it work
fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(!t, f); //Changing t to true

    println!("Success!");
}

