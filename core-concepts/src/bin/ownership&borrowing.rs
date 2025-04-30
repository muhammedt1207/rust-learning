//Ownership

let a=String::from("hello");
let b=a; //owner ship moves
 // println!("{}", a); ❌ error! a no longer valid

//Rust avoids double-free by transferring ownership.

//Borrowing

fn print_msg(msg: &String) {
    println!("{}", msg);
}

fn main() {
    let s = String::from("hi");
    print_msg(&s); // borrow it
    println!("{}", s); // still usable
}

//Immutable borrow: &T