struct User {
    name: String,
    age: u8,
}

fn main() {
    let u = User {
        name: String::from("Bob"),
        age: 25,
    };

    println!("{} is {} years old", u.name, u.age);
}
