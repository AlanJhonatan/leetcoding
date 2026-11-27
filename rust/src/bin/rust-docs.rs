// string and ref intro
fn greet(msg: &str) {
    println!("{msg}")
}

fn main() {
    let mut s = String::from("hello");

    s.push_str(" world!");

    // let view: &str = &s[0..5];

    greet(&s[0..5]);
}

// mutable refs
fn main() {
    let mut s = String::from("hi!");
    let w = &mut s; // this is the mutability owner
    
    // we can do
    // s.push_str("something")
    // but, we can:
    w.push_str(" How are you?");
    // and after that we can
    s.push_str(" wait, this is crazy, hã?");
    

    // we CANT do this again:
    // println!("{w}");
    // Because of:
    // let w = &mut s; // this is the mutability owner
    // |             ------ first mutable borrow occurs here
    // ...
    // 13 |     s.push_str(" wait, this is crazy, hã?");
    // |     ^ second mutable borrow occurs here
    // 14 |
    // 15 |     println!("{w}");
    // |                - first borrow later used here
    // but this is ok, and works fine
    println!("{s}"); // hi! How are you? wait, this is crazy, hã?
}



// copy traits
fn main() {
    let a = String::from("phone");
    let b = a;

    // println!("{a}") // - move occurs because `a` has type `String`, which does not implement the `Copy` trait 
    println!("{b}");
}