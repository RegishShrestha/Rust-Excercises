
//5. Fix errors and panics to make it work
fn main() {
   let v1 = 240_u8 + 8; //Changed to remove integer overflow
   let v2 = i8::checked_add(119, 8).unwrap(); //Changed to remove integer overflow 
   println!("{},{}",v1,v2);
}

