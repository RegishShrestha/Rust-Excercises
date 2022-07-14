
//8. Fix the error below with least amount of modification
fn main() {
    let  (mut x, y) = (1, 2); //Made x variable mutable 
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

