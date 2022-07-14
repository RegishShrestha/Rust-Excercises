//6.
fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead 
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}

