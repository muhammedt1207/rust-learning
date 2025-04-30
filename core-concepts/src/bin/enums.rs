enum Role{
    Guest,
    Admin,
    User(String)
}

fn main(){
    let r=Role::User(String::from("John"));

    match r{
        Role::Guest=> println!("Guest"),
        Role::Admin=>println!("Admin"),
        Role::User(name)=>println!("User: {}",name)
    }
}

//Enums define a value that could be one of several types/variants.

// TypeScript version:

// type Role = "Admin" | "Guest" | { User: string };
// const r: Role = { User: "John" };