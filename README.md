## All Rust libraries and binaries created by Ben Santora. All were coded using AI: ChatGPT - Widenex Rust GPT - Microsoft CoPilot

## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# EnvSwapper

`env_swapper` is a lightweight Rust library to temporarily set environment variables, automatically restoring the previous state when out of scope.

## Installation

Add `env_swapper` to your `Cargo.toml`:

```toml
[dependencies]
env_swapper = "0.1.0"
```

## Usage
```rust
use env_swapper::EnvSwapper;
use std::env;

fn main() {
    let key = "EXAMPLE_VAR";
    
    {
        let _swapper = EnvSwapper::new(&[(key, "temporary_value")]);
        println!("Inside scope: {:?}", env::var(key)); // Outputs "temporary_value"
    }

    println!("Outside scope: {:?}", env::var(key)); // Restored to original
}
```

## Features
Temporarily sets and restores environment variables automatically on scope exit.
Ideal for testing, sandboxed configurations, and controlled environment changes.

## License
This project is licensed under the MIT License.

## Author
Ben Santora 
