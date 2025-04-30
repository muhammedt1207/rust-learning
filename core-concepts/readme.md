
# 🦀 Rust Learning: Core Concepts

A beginner-friendly Rust project exploring core concepts like functions, modules, enums, and project structure — especially for developers transitioning from JavaScript/Node.js.

---

## 📁 Folder Structure

core-concepts/ ├── Cargo.toml ├── src/ │ ├── main.rs // Default entry point │ ├── functions.rs // Custom module │ ├── bin/ // Additional CLI apps │ │ ├── enums.rs // Enum usage demo


---

## 📘 1. Functions in Rust

**src/functions.rs**
```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn say_message() {
    let message = greet("Alice");
    println!("{}", message);
}
src/main.rs

rust
Copy
Edit
mod functions;

fn main() {
    functions::say_message();
}
✅ greet is private, say_message is public.
✅ &str is a borrowed string slice (like passing by reference in JS).
✅ format! is like a String builder: similar to JS template literals.


---



🧱 2. Modules
mod filename; declares a module

Call functions using filename::function()

All .rs files must be in the src/ folder or src/bin/ for binaries


---

🧾 3. Enums and Pattern Matching
src/bin/enums.rs

rust
Copy
Edit
enum Role {
    Guest,
    Admin,
    User(String),
}

fn main() {
    let r = Role::User(String::from("John"));

    match r {
        Role::Guest => println!("Guest"),
        Role::Admin => println!("Admin"),
        Role::User(name) => println!("User: {}", name),
    }
}
To run:


cargo run --bin enums
⚠️ If you see:

typescript
Copy
Edit
warning: variants `Guest` and `Admin` are never constructed
That’s okay — it just means those enum variants weren’t used yet.


---


🧪 4. Running Multiple Files Like Node.js
In Node.js:


node main.js
node task.js
In Rust:

Put files like task.rs, enums.rs, etc. inside src/bin/

Run them using:

bash
Copy
Edit
cargo run --bin task
cargo run --bin enums
Each file in src/bin/ must have its own fn main().

💡 5. Key Syntax & Concepts

Rust	What it Means	JavaScript Equivalent
println!()	Macro that prints to console	console.log()
format!("Hi {}", x)	Formats a string with placeholders	`Hi ${x}`
&str	Borrowed string slice (ref only, not owned)	Strings (always ref)
! after println!	Denotes a macro, not a normal function	No equivalent
fn main()	Required entry point for Rust binary	Your main.js entry file
🧰 Useful Cargo Commands
bash
Copy
Edit
cargo new project-name     # Create new Rust project
cargo run                  # Run main.rs
cargo run --bin enums      # Run a file from src/bin/
cargo check                # Syntax check
cargo build                # Compile the project
✅ Next Steps
Accept user input using std::io

Explore Result<T, E> and error handling

Write your own library using lib.rs

Explore structs, traits, and lifetimes

Made with ❤️ by a JavaScript dev exploring Rust.

yaml
Copy
Edit

---

Just save this as `README.md` in your root project folder (`core-concepts/`), commit, and push to Gi