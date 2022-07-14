1.


fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}


fn main() {
    let x = 10;
    let y = x;
    println!("{},{}",x,y);
}


fn main() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}



fn main() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}





2.


// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}


3.

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    let _s = s.as_bytes();
    s
}


fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    s
}


4.
// Fix the error without removing code line

fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}


5.

// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}


6.


fn main() {
    let s = String::from("hello, ");
    
    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")
}


7.


fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);       // implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);
    println!("Success!");
}


8.


fn main() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // modify this line only, don't use `_s`
    println!("{:?}", t.1);
 }


9.


fn main() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

