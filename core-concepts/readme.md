# 🦀 Rust Learning: Core Concepts

A beginner-friendly Rust project exploring core concepts like functions, modules, enums, and project structure — especially for developers transitioning from JavaScript/Node.js.

## 📁 Project Structure

```
core-concepts/
├── Cargo.toml          # Project configuration file
├── src/                # Source code directory
│   ├── main.rs         # Default entry point
│   ├── functions.rs    # Custom module
│   ├── bin/            # Additional CLI apps
│   │   └── enums.rs    # Enum usage demo
```

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
```

**src/main.rs**
```rust
mod functions;

fn main() {
    functions::say_message();
}
```

✅ `greet` is private, `say_message` is public.  
✅ `&str` is a borrowed string slice (like passing by reference in JS).  
✅ `format!` is like a String builder: similar to JS template literals.

## 🧱 2. Modules

- `mod filename;` declares a module
- Call functions using `filename::function()`
- All `.rs` files must be in the `src/` folder or `src/bin/` for binaries

## 🧾 3. Enums and Pattern Matching

**src/bin/enums.rs**
```rust
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
```

To run:
```bash
cargo run --bin enums
```

⚠️ If you see:
```
warning: variants `Guest` and `Admin` are never constructed
```
That's okay — it just means those enum variants weren't used yet.

## 🧪 4. Running Multiple Files Like Node.js

In Node.js:
```bash
node main.js
node task.js
```

In Rust:
- Put files like `task.rs`, `enums.rs`, etc. inside `src/bin/`
- Run them using:
```bash
cargo run --bin task
cargo run --bin enums
```
- Each file in `src/bin/` must have its own `fn main()`.

## 💡 5. Key Syntax & Concepts

| Rust | What it Means | JavaScript Equivalent |
|------|---------------|----------------------|
| `println!()` | Macro that prints to console | `console.log()` |
| `format!("Hi {}", x)` | Formats a string with placeholders | `` `Hi ${x}` `` |
| `&str` | Borrowed string slice (ref only, not owned) | Strings (always ref) |
| `!` after `println!` | Denotes a macro, not a normal function | No equivalent |
| `fn main()` | Required entry point for Rust binary | Your main.js entry file |

## 🔄 6. Path Separator: `::`

In Rust, the `::` operator is called the path separator and is used to access items inside modules, enums, structs, traits, or crates.

- **Example 1:** Access functions or modules
```rust
let num = i32::from_str_radix("1A", 16).unwrap();
```

- **Example 2:** Access enum variants
```rust
let dir = Direction::Up;
```

- **Example 3:** Use items from a module
```rust
use std::collections::HashMap;
let mut map = HashMap::new();
```

- **Comparison with JS:** In JS you use dot notation (`fs.readFile`), in Rust you use `::` (`fs::read_to_string`)

## 🛡️ 7. Error Handling: `Result<T, E>`

In Rust, there is no try/catch like in JavaScript. Instead, Rust uses a type called `Result<T, E>` for functions that might fail.

```rust
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

## 🧰 8. Tooling: `cargo`

`cargo` is Rust's official package manager, build tool, and project manager—all in one.

**Useful Commands:**
```bash
cargo new project-name     # Create new Rust project
cargo run                  # Run main.rs
cargo run --bin enums      # Run a file from src/bin/
cargo check                # Syntax check
cargo build                # Compile the project
cargo build --release      # Compile with optimizations
cargo add crate_name       # Add a dependency
cargo test                 # Run tests
cargo fmt                  # Format code
```

## 🔍 Rust vs JavaScript Quick Reference

| Concept | Rust | JavaScript/TypeScript |
|---------|------|----------------------|
| Function | `fn greet(name: &str) -> String { ... }` | `function greet(name: string): string { ... }` |
| Struct | `struct User { name: String, age: u8 }` | `type User = { name: string, age: number }` |
| Enum | `enum Role { Admin, User(String) }` | `type Role = "Admin" \| { User: string }` |
| Trait | `trait Greet { fn say_hello(&self); }` | `interface Greet { sayHello(): void }` |
| Ownership | `let b = a;` (a moved) | No JS equivalent |
| Borrowing | `&s`, `&mut s` | No JS equivalent |

## ✅ Next Steps

- Accept user input using `std::io`
- Explore `Result<T, E>` and error handling in depth
- Write your own library using `lib.rs`
- Explore structs, traits, and lifetimes

Made with ❤️ by a JavaScript dev exploring Rust.